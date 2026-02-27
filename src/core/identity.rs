use anyhow::{Context, Result};
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret};
use zeroize::Zeroize;

/// Holds all cryptographic identity material for the local user.
///
/// - Ed25519 keypair: used for message authentication and as the public identity.
/// - X25519 keypair: used for Diffie-Hellman key exchange.
#[derive(Clone)]
pub struct Identity {
    /// Ed25519 signing key (private)
    pub signing_key: SigningKey,
    /// Ed25519 verifying key (public identity — shared with others)
    pub verifying_key: ed25519_dalek::VerifyingKey,
    /// X25519 static secret (private — for DH key exchange)
    pub dh_secret: StaticSecret,
    /// X25519 public key (shared for key agreement)
    pub dh_public: X25519PublicKey,
}

/// Serializable representation of identity material for storage.
#[derive(Serialize, Deserialize, Zeroize)]
#[zeroize(drop)]
struct IdentityData {
    signing_key_bytes: Vec<u8>,
    dh_secret_bytes: Vec<u8>,
}

impl Identity {
    /// Generate a brand-new random identity.
    pub fn generate() -> Self {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();

        let dh_secret = StaticSecret::random_from_rng(OsRng);
        let dh_public = X25519PublicKey::from(&dh_secret);

        Self {
            signing_key,
            verifying_key,
            dh_secret,
            dh_public,
        }
    }

    /// Returns the public identity as a hex string with `wraith:` prefix.
    pub fn public_key_hex(&self) -> String {
        format!("wraith:{}", hex::encode(self.verifying_key.as_bytes()))
    }

    /// Returns just the raw hex of the verifying key (no prefix).
    pub fn public_key_raw_hex(&self) -> String {
        hex::encode(self.verifying_key.as_bytes())
    }

    /// Returns a short truncated display form.
    pub fn public_key_short(&self) -> String {
        let full = self.public_key_raw_hex();
        if full.len() > 12 {
            format!("wraith:{}...{}", &full[..6], &full[full.len() - 6..])
        } else {
            format!("wraith:{}", full)
        }
    }

    /// Persist identity to the sled database.
    pub fn save_to_storage(&self, db: &sled::Db) -> Result<()> {
        let data = IdentityData {
            signing_key_bytes: self.signing_key.to_bytes().to_vec(),
            dh_secret_bytes: self.dh_secret.to_bytes().to_vec(),
        };
        let encoded = bincode::serialize(&data).context("Failed to serialize identity")?;
        db.insert("identity:keypair", encoded)
            .context("Failed to write identity to storage")?;
        db.flush().context("Failed to flush storage")?;
        Ok(())
    }

    /// Load identity from the sled database.
    pub fn load_from_storage(db: &sled::Db) -> Result<Option<Self>> {
        match db.get("identity:keypair").context("Failed to read identity from storage")? {
            Some(bytes) => {
                let data: IdentityData =
                    bincode::deserialize(&bytes).context("Failed to deserialize identity")?;

                let signing_key_bytes: [u8; 32] = data
                    .signing_key_bytes
                    .as_slice()
                    .try_into()
                    .context("Invalid signing key length")?;
                let signing_key = SigningKey::from_bytes(&signing_key_bytes);
                let verifying_key = signing_key.verifying_key();

                let dh_secret_bytes: [u8; 32] = data
                    .dh_secret_bytes
                    .as_slice()
                    .try_into()
                    .context("Invalid DH secret length")?;
                let dh_secret = StaticSecret::from(dh_secret_bytes);
                let dh_public = X25519PublicKey::from(&dh_secret);

                Ok(Some(Self {
                    signing_key,
                    verifying_key,
                    dh_secret,
                    dh_public,
                }))
            }
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_identity() {
        let id = Identity::generate();
        let hex_key = id.public_key_hex();
        assert!(hex_key.starts_with("wraith:"));
        assert_eq!(hex_key.len(), 7 + 64); // "wraith:" + 64 hex chars for 32 bytes
    }

    #[test]
    fn test_short_key() {
        let id = Identity::generate();
        let short = id.public_key_short();
        assert!(short.starts_with("wraith:"));
        assert!(short.contains("..."));
    }

    #[test]
    fn test_save_load_roundtrip() {
        let db = sled::Config::new().temporary(true).open().unwrap();
        let id = Identity::generate();
        id.save_to_storage(&db).unwrap();

        let loaded = Identity::load_from_storage(&db).unwrap().unwrap();
        assert_eq!(id.public_key_hex(), loaded.public_key_hex());
        assert_eq!(
            hex::encode(id.dh_public.as_bytes()),
            hex::encode(loaded.dh_public.as_bytes())
        );
    }
}
