use dioxus::prelude::*;

use crate::state::app_state::AppState;
use crate::ui::screens::{
    splash::Splash,
    onboarding::Onboarding,
    home::Home,
    chat::Chat,
    new_chat::NewChat,
    profile::Profile,
    settings::Settings,
};

/// Application routes.
#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Splash {},
    #[route("/onboarding")]
    Onboarding {},
    #[route("/home")]
    Home {},
    #[route("/chat/:contact_id")]
    Chat { contact_id: String },
    #[route("/new-chat")]
    NewChat {},
    #[route("/profile")]
    Profile {},
    #[route("/settings")]
    Settings {},
}

/// Root application component.
///
/// Provides global state via context and renders the router.
#[component]
pub fn App() -> Element {
    let mut state = use_context_provider(|| Signal::new(AppState::new()));
    let mut state_clone = state;

    // Initialize storage asynchronously on startup
    use_coroutine(move |_rx: UnboundedReceiver<()>| async move {
        // Find a sensible data directory across platforms
        let data_dir = dirs::data_local_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("wraith_db");

        // Ensure the directory exists
        if let Err(e) = std::fs::create_dir_all(&data_dir) {
            tracing::error!("Failed to create data directory: {}", e);
        }

        match crate::core::storage::AppStorage::open(data_dir) {
            Ok(storage) => {
                let arc_storage = std::sync::Arc::new(storage);
                
                // Load saved data into state
                let mut contacts = Vec::new();
                let mut messages = std::collections::HashMap::new();
                let mut identity = None;
                
                if let Ok(loaded_contacts) = arc_storage.load_all_contacts() {
                    contacts = loaded_contacts;
                }
                
                // Load messages for each contact we have
                for contact in &contacts {
                    if let Ok(loaded_msgs) = arc_storage.load_thread_messages(&contact.public_key) {
                        messages.insert(contact.public_key.clone(), loaded_msgs);
                    }
                }
                
                if let Ok(loaded_identity) = crate::core::identity::Identity::load_from_storage(arc_storage.db()) {
                    identity = loaded_identity;
                }

                // Update the state all at once
                state_clone.with_mut(|s| {
                    s.storage = Some(arc_storage);
                    s.contacts = contacts;
                    s.messages = messages;
                    s.identity = identity;
                    s.is_loading = false;
                    // If we have an identity, assume onboarded for now
                    if s.identity.is_some() {
                        s.onboarded = true;
                    }
                });
                tracing::info!("Storage initialized successfully");
            }
            Err(e) => {
                tracing::error!("Failed to open storage: {:?}", e);
                // Even on error, we stop loading so the UI can show an error or fallback
                state_clone.with_mut(|s| s.is_loading = false);
            }
        }
    });
    // Simulate Tor Connection Process
    let mut tor_state = state;
    use_coroutine(move |_rx: UnboundedReceiver<()>| async move {
        // Start by showing disconnected
        tor_state.with_mut(|s| s.tor_status = crate::core::types::TorStatus::Disconnected);
        
        #[cfg(not(target_family = "wasm"))]
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        
        // Move to connecting state
        tor_state.with_mut(|s| s.tor_status = crate::core::types::TorStatus::Connecting);
        
        #[cfg(not(target_family = "wasm"))]
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        
        // Finally connected
        tor_state.with_mut(|s| s.tor_status = crate::core::types::TorStatus::Connected);
    });

    rsx! {
        style { {include_str!("../assets/global.css")} }
        Router::<Route> {}
    }
}
