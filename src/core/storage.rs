use anyhow::{Context, Result};
use serde::{de::DeserializeOwned, Serialize};
use std::path::PathBuf;

use super::types::{AppSettings, Contact, Message};

/// Encrypted key-value store backed by sled.
///
/// All data is namespaced:
/// - `identity:keypair` → Identity
/// - `contacts:{pubkey}` → Contact
/// - `messages:{thread_id}:{timestamp}` → Message
/// - `settings:*` → Settings values
#[derive(Clone)]
pub struct AppStorage {
    db: sled::Db,
}

impl AppStorage {
    /// Open or create the storage database at the given path.
    pub fn open(path: PathBuf) -> Result<Self> {
        let db = sled::open(&path)
            .with_context(|| format!("Failed to open database at {:?}", path))?;
        tracing::info!("Storage opened at {:?}", path);
        Ok(Self { db })
    }

    /// Open a temporary in-memory database (for testing).
    pub fn open_temporary() -> Result<Self> {
        let db = sled::Config::new()
            .temporary(true)
            .open()
            .context("Failed to open temporary database")?;
        Ok(Self { db })
    }

    /// Get a reference to the underlying sled::Db.
    pub fn db(&self) -> &sled::Db {
        &self.db
    }

    // ─── Contacts ────────────────────────────────────────────

    /// Save a contact.
    pub fn save_contact(&self, contact: &Contact) -> Result<()> {
        let key = format!("contacts:{}", contact.public_key);
        let value = bincode::serialize(contact).context("Failed to serialize contact")?;
        self.db.insert(key.as_bytes(), value).context("Failed to save contact")?;
        Ok(())
    }

    /// Load a contact by public key.
    pub fn load_contact(&self, public_key: &str) -> Result<Option<Contact>> {
        let key = format!("contacts:{}", public_key);
        self.get_deserialized(&key)
    }

    /// Load all contacts.
    pub fn load_all_contacts(&self) -> Result<Vec<Contact>> {
        let mut contacts = Vec::new();
        for item in self.db.scan_prefix(b"contacts:") {
            let (_, value) = item.context("Failed to scan contacts")?;
            let contact: Contact =
                bincode::deserialize(&value).context("Failed to deserialize contact")?;
            contacts.push(contact);
        }
        // Sort by last message time, most recent first
        contacts.sort_by(|a, b| b.last_message_time.cmp(&a.last_message_time));
        Ok(contacts)
    }

    /// Delete a contact by public key.
    pub fn delete_contact(&self, public_key: &str) -> Result<()> {
        let key = format!("contacts:{}", public_key);
        self.db.remove(key.as_bytes()).context("Failed to delete contact")?;
        Ok(())
    }

    // ─── Messages ────────────────────────────────────────────

    /// Save a message.
    pub fn save_message(&self, message: &Message) -> Result<()> {
        let key = format!(
            "messages:{}:{}",
            message.thread_id,
            message.timestamp.timestamp_millis()
        );
        let value = bincode::serialize(message).context("Failed to serialize message")?;
        self.db.insert(key.as_bytes(), value).context("Failed to save message")?;
        Ok(())
    }

    /// Load all messages for a thread, ordered by timestamp.
    pub fn load_thread_messages(&self, thread_id: &str) -> Result<Vec<Message>> {
        let prefix = format!("messages:{}:", thread_id);
        let mut messages = Vec::new();
        for item in self.db.scan_prefix(prefix.as_bytes()) {
            let (_, value) = item.context("Failed to scan messages")?;
            let message: Message =
                bincode::deserialize(&value).context("Failed to deserialize message")?;
            messages.push(message);
        }
        messages.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        Ok(messages)
    }

    /// Delete all messages in a thread.
    pub fn delete_thread(&self, thread_id: &str) -> Result<()> {
        let prefix = format!("messages:{}:", thread_id);
        for item in self.db.scan_prefix(prefix.as_bytes()) {
            let (key, _) = item.context("Failed to scan messages for deletion")?;
            self.db.remove(key).context("Failed to delete message")?;
        }
        Ok(())
    }

    // ─── Settings ────────────────────────────────────────────

    /// Save application settings.
    pub fn save_settings(&self, settings: &AppSettings) -> Result<()> {
        let value = bincode::serialize(settings).context("Failed to serialize settings")?;
        self.db
            .insert(b"settings:app", value)
            .context("Failed to save settings")?;
        Ok(())
    }

    /// Load application settings.
    pub fn load_settings(&self) -> Result<AppSettings> {
        match self.db.get(b"settings:app").context("Failed to read settings")? {
            Some(bytes) => {
                bincode::deserialize(&bytes).context("Failed to deserialize settings")
            }
            None => Ok(AppSettings::default()),
        }
    }

    // ─── Utility ─────────────────────────────────────────────

    /// Wipe all data from the database.
    pub fn wipe_all(&self) -> Result<()> {
        self.db.clear().context("Failed to wipe database")?;
        self.db.flush().context("Failed to flush after wipe")?;
        tracing::info!("All data wiped from storage");
        Ok(())
    }

    /// Flush pending writes to disk.
    pub fn flush(&self) -> Result<()> {
        self.db.flush().context("Failed to flush storage")?;
        Ok(())
    }

    /// Helper: get and deserialize a value by string key.
    fn get_deserialized<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>> {
        match self
            .db
            .get(key.as_bytes())
            .with_context(|| format!("Failed to read key: {}", key))?
        {
            Some(bytes) => {
                let value = bincode::deserialize(&bytes)
                    .with_context(|| format!("Failed to deserialize key: {}", key))?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::MessageStatus;

    #[test]
    fn test_contact_crud() {
        let storage = AppStorage::open_temporary().unwrap();

        let contact = Contact {
            id: uuid::Uuid::new_v4().to_string(),
            public_key: "abc123def456".to_string(),
            nickname: Some("Alice".to_string()),
            last_seen: None,
            unread_count: 0,
            last_message_preview: None,
            last_message_time: None,
        };

        storage.save_contact(&contact).unwrap();
        let loaded = storage.load_contact("abc123def456").unwrap().unwrap();
        assert_eq!(loaded.nickname, Some("Alice".to_string()));

        let all = storage.load_all_contacts().unwrap();
        assert_eq!(all.len(), 1);

        storage.delete_contact("abc123def456").unwrap();
        let deleted = storage.load_contact("abc123def456").unwrap();
        assert!(deleted.is_none());
    }

    #[test]
    fn test_message_thread() {
        let storage = AppStorage::open_temporary().unwrap();

        let thread_id = "thread_abc";
        for i in 0..3 {
            let msg = Message {
                id: uuid::Uuid::new_v4().to_string(),
                thread_id: thread_id.to_string(),
                sender_key: "sender_key".to_string(),
                content: format!("Message {}", i),
                timestamp: chrono::Utc::now() + chrono::Duration::seconds(i as i64),
                status: MessageStatus::Sent,
            };
            storage.save_message(&msg).unwrap();
        }

        let messages = storage.load_thread_messages(thread_id).unwrap();
        assert_eq!(messages.len(), 3);
        assert!(messages[0].timestamp <= messages[1].timestamp);
    }

    #[test]
    fn test_settings() {
        let storage = AppStorage::open_temporary().unwrap();

        // Default settings
        let settings = storage.load_settings().unwrap();
        assert!(!settings.passphrase_lock_enabled);

        // Save custom settings
        let mut custom = AppSettings::default();
        custom.passphrase_lock_enabled = true;
        storage.save_settings(&custom).unwrap();

        let loaded = storage.load_settings().unwrap();
        assert!(loaded.passphrase_lock_enabled);
    }

    #[test]
    fn test_wipe_all() {
        let storage = AppStorage::open_temporary().unwrap();
        let contact = Contact {
            id: "test".to_string(),
            public_key: "key123".to_string(),
            nickname: None,
            last_seen: None,
            unread_count: 0,
            last_message_preview: None,
            last_message_time: None,
        };
        storage.save_contact(&contact).unwrap();
        storage.wipe_all().unwrap();
        let all = storage.load_all_contacts().unwrap();
        assert!(all.is_empty());
    }
}
