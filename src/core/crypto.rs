use anyhow::{Context, Result};
use chacha20poly1305::{
    aead::{Aead, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce,
};
use hkdf::Hkdf;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret};
use zeroize::Zeroize;

/// An encrypted message ready for transport.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedMessage {
    /// ChaCha20-Poly1305 ciphertext + auth tag
    pub ciphertext: Vec<u8>,
    /// 12-byte nonce used for encryption
    pub nonce: [u8; 12],
    /// Sender's current DH ratchet public key
    pub dh_public: [u8; 32],
    /// Message number in the current send chain
    pub message_number: u32,
}

/// A simplified Double Ratchet session for E2E encryption.
///
/// Manages root key, send/receive chain keys, and DH ratchet keys
/// to provide forward secrecy for each message.
pub struct RatchetSession {
    /// Root key — used to derive new chain keys on DH ratchet step
    pub root_key: [u8; 32],
    /// Current send chain key
    pub send_chain_key: [u8; 32],
    /// Current receive chain key
    pub recv_chain_key: [u8; 32],
    /// Number of messages sent in current chain
    pub send_message_number: u32,
    /// Number of messages received in current chain
    pub recv_message_number: u32,
    /// Our current DH ratchet secret
    pub dh_ratchet_key: StaticSecret,
    /// The remote party's current DH public key
    pub remote_dh_public: X25519PublicKey,
}

impl RatchetSession {
    /// Initialize a new ratchet session from a shared secret (from X25519 key agreement).
    pub fn initialize(
        shared_secret: [u8; 32],
        our_dh: StaticSecret,
        their_dh_public: X25519PublicKey,
    ) -> Self {
        // Derive initial root, send chain, and recv chain keys from the shared secret
        let hk = Hkdf::<Sha256>::new(Some(b"wraith-ratchet-init"), &shared_secret);

        let mut root_key = [0u8; 32];
        let mut send_chain_key = [0u8; 32];
        let mut recv_chain_key = [0u8; 32];

        hk.expand(b"wraith-root-key", &mut root_key)
            .expect("HKDF expand failed");
        hk.expand(b"wraith-send-chain", &mut send_chain_key)
            .expect("HKDF expand failed");
        hk.expand(b"wraith-recv-chain", &mut recv_chain_key)
            .expect("HKDF expand failed");

        Self {
            root_key,
            send_chain_key,
            recv_chain_key,
            send_message_number: 0,
            recv_message_number: 0,
            dh_ratchet_key: our_dh,
            remote_dh_public: their_dh_public,
        }
    }

    /// Encrypt a plaintext message, advancing the send chain.
    pub fn encrypt(&mut self, plaintext: &[u8]) -> Result<EncryptedMessage> {
        // Derive message key from send chain key
        let message_key = self.ratchet_send();

        // Generate a random nonce
        let mut nonce_bytes = [0u8; 12];
        rand::rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Encrypt with ChaCha20-Poly1305
        let cipher = ChaCha20Poly1305::new_from_slice(&message_key)
            .context("Failed to create cipher")?;
        let ciphertext = cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

        let dh_public_bytes = *X25519PublicKey::from(&self.dh_ratchet_key).as_bytes();

        let msg = EncryptedMessage {
            ciphertext,
            nonce: nonce_bytes,
            dh_public: dh_public_bytes,
            message_number: self.send_message_number - 1,
        };

        Ok(msg)
    }

    /// Decrypt an incoming encrypted message, advancing the receive chain.
    pub fn decrypt(&mut self, msg: &EncryptedMessage) -> Result<Vec<u8>> {
        // Check if a DH ratchet step is needed
        let sender_public = X25519PublicKey::from(msg.dh_public);
        if sender_public.as_bytes() != self.remote_dh_public.as_bytes() {
            self.ratchet_recv(sender_public);
        }

        // Derive message key from receive chain key
        let message_key = self.derive_recv_message_key();

        let nonce = Nonce::from_slice(&msg.nonce);
        let cipher = ChaCha20Poly1305::new_from_slice(&message_key)
            .context("Failed to create cipher")?;
        let plaintext = cipher
            .decrypt(nonce, msg.ciphertext.as_ref())
            .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;

        self.recv_message_number += 1;

        Ok(plaintext)
    }

    /// Advance the send chain and return a message key.
    fn ratchet_send(&mut self) -> [u8; 32] {
        let hk = Hkdf::<Sha256>::new(Some(&self.root_key), &self.send_chain_key);
        let mut message_key = [0u8; 32];
        let mut new_chain_key = [0u8; 32];

        hk.expand(b"wraith-msg-key", &mut message_key)
            .expect("HKDF expand failed");
        hk.expand(b"wraith-chain-advance", &mut new_chain_key)
            .expect("HKDF expand failed");

        self.send_chain_key = new_chain_key;
        self.send_message_number += 1;

        message_key
    }

    /// Derive receive message key and advance receive chain.
    fn derive_recv_message_key(&mut self) -> [u8; 32] {
        let hk = Hkdf::<Sha256>::new(Some(&self.root_key), &self.recv_chain_key);
        let mut message_key = [0u8; 32];
        let mut new_chain_key = [0u8; 32];

        hk.expand(b"wraith-msg-key", &mut message_key)
            .expect("HKDF expand failed");
        hk.expand(b"wraith-chain-advance", &mut new_chain_key)
            .expect("HKDF expand failed");

        self.recv_chain_key = new_chain_key;

        message_key
    }

    /// Perform a DH ratchet step when the remote party's key changes.
    fn ratchet_recv(&mut self, new_remote_public: X25519PublicKey) {
        self.remote_dh_public = new_remote_public;

        // Compute new shared secret
        let dh_output = self.dh_ratchet_key.diffie_hellman(&new_remote_public);

        // Derive new root key and receive chain key
        let hk = Hkdf::<Sha256>::new(Some(&self.root_key), dh_output.as_bytes());

        let mut new_root_key = [0u8; 32];
        let mut new_recv_chain_key = [0u8; 32];

        hk.expand(b"wraith-root-ratchet", &mut new_root_key)
            .expect("HKDF expand failed");
        hk.expand(b"wraith-recv-ratchet", &mut new_recv_chain_key)
            .expect("HKDF expand failed");

        self.root_key = new_root_key;
        self.recv_chain_key = new_recv_chain_key;
        self.recv_message_number = 0;

        // Generate new DH ratchet keypair for our side
        let new_dh = StaticSecret::random_from_rng(OsRng);
        let new_dh_output = new_dh.diffie_hellman(&new_remote_public);

        let hk2 = Hkdf::<Sha256>::new(Some(&self.root_key), new_dh_output.as_bytes());

        let mut new_root_key2 = [0u8; 32];
        let mut new_send_chain_key = [0u8; 32];

        hk2.expand(b"wraith-root-ratchet", &mut new_root_key2)
            .expect("HKDF expand failed");
        hk2.expand(b"wraith-send-ratchet", &mut new_send_chain_key)
            .expect("HKDF expand failed");

        self.root_key = new_root_key2;
        self.send_chain_key = new_send_chain_key;
        self.send_message_number = 0;
        self.dh_ratchet_key = new_dh;
    }
}

impl Drop for RatchetSession {
    fn drop(&mut self) {
        self.root_key.zeroize();
        self.send_chain_key.zeroize();
        self.recv_chain_key.zeroize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_session_pair() -> (RatchetSession, RatchetSession) {
        // Simulate two parties performing X25519 key exchange
        let alice_dh = StaticSecret::random_from_rng(OsRng);
        let alice_dh_pub = X25519PublicKey::from(&alice_dh);

        let bob_dh = StaticSecret::random_from_rng(OsRng);
        let bob_dh_pub = X25519PublicKey::from(&bob_dh);

        // Both compute the same shared secret
        let alice_shared = alice_dh.diffie_hellman(&bob_dh_pub);
        let bob_shared = bob_dh.diffie_hellman(&alice_dh_pub);
        assert_eq!(alice_shared.as_bytes(), bob_shared.as_bytes());

        let mut shared = [0u8; 32];
        shared.copy_from_slice(alice_shared.as_bytes());

        // Alice is the initiator
        let alice_ratchet_dh = StaticSecret::random_from_rng(OsRng);
        let alice_ratchet_pub = X25519PublicKey::from(&alice_ratchet_dh);

        let bob_ratchet_dh = StaticSecret::random_from_rng(OsRng);
        let bob_ratchet_pub = X25519PublicKey::from(&bob_ratchet_dh);

        let alice_session =
            RatchetSession::initialize(shared, alice_ratchet_dh, bob_ratchet_pub);
        let bob_session = RatchetSession::initialize(shared, bob_ratchet_dh, alice_ratchet_pub);

        (alice_session, bob_session)
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let (mut alice, mut bob) = create_session_pair();

        let plaintext = b"Hello from Alice!";
        let encrypted = alice.encrypt(plaintext).unwrap();
        let decrypted = bob.decrypt(&encrypted).unwrap();

        assert_eq!(plaintext.to_vec(), decrypted);
    }

    #[test]
    fn test_multiple_messages() {
        let (mut alice, mut bob) = create_session_pair();

        for i in 0..5 {
            let msg = format!("Message {}", i);
            let encrypted = alice.encrypt(msg.as_bytes()).unwrap();
            let decrypted = bob.decrypt(&encrypted).unwrap();
            assert_eq!(msg.as_bytes(), decrypted.as_slice());
        }
    }
}
