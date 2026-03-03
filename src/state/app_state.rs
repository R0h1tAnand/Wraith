use dioxus::prelude::*;
use std::collections::HashMap;

use crate::core::types::{Contact, Message, TorStatus};
use crate::core::identity::Identity;

/// Global reactive application state.
///
/// Provided at the root component via `use_context_provider`.
/// Accessed in children via `use_context::<Signal<AppState>>()`.
#[derive(Clone)]
pub struct AppState {
    /// Current Tor network status
    pub tor_status: TorStatus,
    /// Tor Client instance
    pub tor_client: Option<crate::core::tor::AppTorClient>,
    /// All contacts
    pub contacts: Vec<Contact>,
    /// Currently active chat thread contact ID
    pub active_thread: Option<String>,
    /// Messages indexed by thread_id
    pub messages: HashMap<String, Vec<Message>>,
    /// Local user identity (None until generated/loaded)
    pub identity: Option<Identity>,
    /// Persistent storage engine
    pub storage: Option<std::sync::Arc<crate::core::storage::AppStorage>>,
    /// Whether the app is loading
    pub is_loading: bool,
    /// Whether the user has completed onboarding
    pub onboarded: bool,
    /// Search query on home screen
    pub search_query: String,
}

impl AppState {
    /// Create initial app state.
    pub fn new() -> Self {
        Self {
            tor_status: TorStatus::Disconnected,
            tor_client: None,
            contacts: Vec::new(),
            active_thread: None,
            messages: HashMap::new(),
            identity: None,
            storage: None,
            is_loading: true,
            onboarded: false,
            search_query: String::new(),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
