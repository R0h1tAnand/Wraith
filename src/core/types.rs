use serde::{Deserialize, Serialize};

/// Represents a contact in the user's contact list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    /// Unique identifier (UUID v4)
    pub id: String,
    /// Hex-encoded Ed25519 verifying key
    pub public_key: String,
    /// Optional display name
    pub nickname: Option<String>,
    /// Last time this contact was seen online
    pub last_seen: Option<chrono::DateTime<chrono::Utc>>,
    /// Number of unread messages from this contact
    pub unread_count: u32,
    /// Preview of the last message (plaintext)
    pub last_message_preview: Option<String>,
    /// Timestamp of the last message
    pub last_message_time: Option<chrono::DateTime<chrono::Utc>>,
}

/// A single message in a conversation thread.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Unique identifier (UUID v4)
    pub id: String,
    /// Thread identifier — derived from sorted contact pubkeys
    pub thread_id: String,
    /// Public key of the sender
    pub sender_key: String,
    /// Decrypted plaintext content (only stored decrypted in memory)
    pub content: String,
    /// When the message was created/sent
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Current delivery status
    pub status: MessageStatus,
}

/// Delivery status of a message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageStatus {
    /// Message is being encrypted and routed
    Sending,
    /// Message was sent over Tor
    Sent,
    /// Message was received by the recipient
    Delivered,
    /// Recipient opened the message
    Read,
    /// Delivery failed
    Failed,
}

/// Current Tor network connection status.
#[derive(Debug, Clone, PartialEq)]
pub enum TorStatus {
    /// Bootstrapping — connecting to guard nodes
    Connecting,
    /// Fully connected with a valid circuit
    Connected,
    /// Connected but circuit quality is poor
    Degraded,
    /// Not connected to Tor
    Disconnected,
}

impl TorStatus {
    /// Returns the display label for the status.
    pub fn label(&self) -> &'static str {
        match self {
            TorStatus::Connecting => "Connecting to Tor...",
            TorStatus::Connected => "Connected via Tor",
            TorStatus::Degraded => "Weak circuit",
            TorStatus::Disconnected => "Not connected",
        }
    }

    /// Returns the status color hex code.
    pub fn color(&self) -> &'static str {
        match self {
            TorStatus::Connecting => "#FBBF24",
            TorStatus::Connected => "#4ADE80",
            TorStatus::Degraded => "#FB923C",
            TorStatus::Disconnected => "#F87171",
        }
    }
}

/// Represents the current screen/navigation state.
#[derive(Debug, Clone, PartialEq)]
pub enum AppScreen {
    Splash,
    Onboarding,
    Home,
    Chat(String), // contact_id
    NewChat,
    Profile,
    Settings,
}

/// Onboarding step tracker.
#[derive(Debug, Clone, PartialEq)]
pub enum OnboardingStep {
    Welcome,
    Generating,
    Backup,
}

/// Settings for auto-clearing messages.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AutoClearDuration {
    SevenDays,
    ThirtyDays,
    Never,
}

impl AutoClearDuration {
    pub fn label(&self) -> &'static str {
        match self {
            AutoClearDuration::SevenDays => "7 days",
            AutoClearDuration::ThirtyDays => "30 days",
            AutoClearDuration::Never => "Never",
        }
    }
}

/// Application settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub passphrase_lock_enabled: bool,
    pub auto_clear: AutoClearDuration,
    pub notifications_enabled: bool,
    pub connection_timeout_secs: u64,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            passphrase_lock_enabled: false,
            auto_clear: AutoClearDuration::Never,
            notifications_enabled: false,
            connection_timeout_secs: 30,
        }
    }
}
