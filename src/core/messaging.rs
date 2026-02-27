use anyhow::Result;
use chrono::Utc;
use uuid::Uuid;

use super::types::{Message, MessageStatus};

/// Derives a thread ID from two public keys (sorted for consistency).
pub fn derive_thread_id(our_key: &str, their_key: &str) -> String {
    let mut keys = [our_key, their_key];
    keys.sort();
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(keys[0].as_bytes());
    hasher.update(b":");
    hasher.update(keys[1].as_bytes());
    let hash = hasher.finalize();
    hex::encode(&hash[..16]) // Use first 16 bytes for a shorter ID
}

/// Create a new outgoing message.
pub fn create_outgoing_message(
    thread_id: &str,
    sender_key: &str,
    content: &str,
) -> Message {
    Message {
        id: Uuid::new_v4().to_string(),
        thread_id: thread_id.to_string(),
        sender_key: sender_key.to_string(),
        content: content.to_string(),
        timestamp: Utc::now(),
        status: MessageStatus::Sending,
    }
}

/// Parse an incoming decrypted message payload.
///
/// Expected wire format (JSON):
/// ```json
/// {
///   "id": "uuid",
///   "thread_id": "...",
///   "sender_key": "hex...",
///   "content": "hello",
///   "timestamp": "2024-01-01T00:00:00Z"
/// }
/// ```
pub fn parse_incoming_message(decrypted_payload: &[u8]) -> Result<Message> {
    let mut msg: Message = serde_json::from_slice(decrypted_payload)?;
    msg.status = MessageStatus::Delivered;
    Ok(msg)
}

/// Serialize a message for sending over the wire.
pub fn serialize_for_wire(message: &Message) -> Result<Vec<u8>> {
    let json = serde_json::to_vec(message)?;
    Ok(json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thread_id_deterministic() {
        let id1 = derive_thread_id("alice_key", "bob_key");
        let id2 = derive_thread_id("bob_key", "alice_key");
        assert_eq!(id1, id2, "Thread ID should be same regardless of key order");
    }

    #[test]
    fn test_create_message() {
        let msg = create_outgoing_message("thread1", "sender_abc", "Hello!");
        assert_eq!(msg.content, "Hello!");
        assert_eq!(msg.status, MessageStatus::Sending);
        assert!(!msg.id.is_empty());
    }

    #[test]
    fn test_message_roundtrip() {
        let msg = create_outgoing_message("thread1", "sender_abc", "Hello!");
        let serialized = serialize_for_wire(&msg).unwrap();
        let parsed = parse_incoming_message(&serialized).unwrap();
        assert_eq!(parsed.content, "Hello!");
        assert_eq!(parsed.status, MessageStatus::Delivered);
    }
}
