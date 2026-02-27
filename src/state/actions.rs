use dioxus::prelude::*;

use crate::core::identity::Identity;
use crate::core::messaging;
use crate::core::types::{Contact, Message, MessageStatus, TorStatus};
use super::app_state::AppState;

/// Generate a new identity and update state.
pub fn action_generate_identity(state: &mut Signal<AppState>) {
    let identity = Identity::generate();
    let mut s = state.write();
    s.identity = Some(identity);
    s.onboarded = true;
    s.is_loading = false;
}

/// Add a new contact by public key.
pub fn action_add_contact(
    state: &mut Signal<AppState>,
    public_key: String,
    nickname: Option<String>,
) {
    let contact = Contact {
        id: uuid::Uuid::new_v4().to_string(),
        public_key,
        nickname,
        last_seen: None,
        unread_count: 0,
        last_message_preview: None,
        last_message_time: None,
    };
    state.write().contacts.push(contact);
}

/// Send a message in the active thread.
pub fn action_send_message(
    state: &mut Signal<AppState>,
    contact_id: &str,
    content: &str,
) {
    let mut s = state.write();

    // Find the contact
    let contact = match s.contacts.iter().find(|c| c.id == contact_id) {
        Some(c) => c.clone(),
        None => return,
    };

    let sender_key = match &s.identity {
        Some(id) => id.public_key_raw_hex(),
        None => return,
    };

    let thread_id = messaging::derive_thread_id(&sender_key, &contact.public_key);
    let message = messaging::create_outgoing_message(&thread_id, &sender_key, content);

    // Update contact's last message info
    if let Some(c) = s.contacts.iter_mut().find(|c| c.id == contact_id) {
        c.last_message_preview = Some(content.to_string());
        c.last_message_time = Some(message.timestamp);
    }

    // Add to messages map
    s.messages
        .entry(thread_id)
        .or_insert_with(Vec::new)
        .push(message);
}

/// Mark messages as read for a given contact.
pub fn action_mark_read(state: &mut Signal<AppState>, contact_id: &str) {
    let mut s = state.write();

    // Reset unread count
    if let Some(c) = s.contacts.iter_mut().find(|c| c.id == contact_id) {
        c.unread_count = 0;
    }
}

/// Delete a contact and all their messages.
pub fn action_delete_contact(state: &mut Signal<AppState>, contact_id: &str) {
    let mut s = state.write();

    // Find and remove the contact
    if let Some(pos) = s.contacts.iter().position(|c| c.id == contact_id) {
        let contact = s.contacts.remove(pos);

        // Also remove their message thread
        if let Some(identity) = &s.identity {
            let thread_id = messaging::derive_thread_id(
                &identity.public_key_raw_hex(),
                &contact.public_key,
            );
            s.messages.remove(&thread_id);
        }
    }
}

/// Update Tor connection status.
pub fn action_update_tor_status(state: &mut Signal<AppState>, status: TorStatus) {
    state.write().tor_status = status;
}

/// Set the active chat thread.
pub fn action_set_active_thread(state: &mut Signal<AppState>, contact_id: Option<String>) {
    state.write().active_thread = contact_id;
}

/// Set search query.
pub fn action_set_search(state: &mut Signal<AppState>, query: String) {
    state.write().search_query = query;
}
