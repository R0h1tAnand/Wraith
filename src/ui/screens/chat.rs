use dioxus::prelude::*;
use crate::app::Route;
use crate::core::messaging;
use crate::state::app_state::AppState;
use crate::state::actions;
use crate::ui::theme::DARK;
use crate::ui::animations::presets;
use crate::ui::components::message_bubble::MessageBubble;
use crate::ui::components::input_bar::InputBar;
use crate::ui::components::avatar::Avatar;

/// Individual chat view.
#[component]
pub fn Chat(contact_id: String) -> Element {
    let state = use_context::<Signal<AppState>>();
    let nav = use_navigator();
    let mut message_input = use_signal(|| String::new());

    // Find the contact
    let contact = state.read().contacts.iter()
        .find(|c| c.id == contact_id)
        .cloned();

    let contact = match contact {
        Some(c) => c,
        None => {
            return rsx! {
                div {
                    style: "
                        width: 100%;
                        height: 100vh;
                        display: flex;
                        flex-direction: column;
                        align-items: center;
                        justify-content: center;
                        background: {DARK.bg_primary};
                        color: {DARK.text_secondary};
                        gap: 12px;
                    ",
                    div { style: "font-size: 48px; opacity: 0.5;", "🔍" }
                    span { style: "font-size: 15px;", "Contact not found" }
                }
            };
        }
    };

    // Get messages for this thread
    let our_key = state.read().identity.as_ref()
        .map(|id| id.public_key_raw_hex())
        .unwrap_or_default();
    let thread_id = messaging::derive_thread_id(&our_key, &contact.public_key);
    let messages = state.read().messages
        .get(&thread_id)
        .cloned()
        .unwrap_or_default();

    let display_name = contact.nickname.as_deref().unwrap_or("Unknown");

    rsx! {
        div {
            style: "
                width: 100%;
                height: 100vh;
                display: flex;
                flex-direction: column;
                background: {DARK.bg_primary};
                position: relative;
            ",

            // ─── Header ─────────────────────────
            div {
                style: "
                    height: 64px;
                    display: flex;
                    align-items: center;
                    gap: 12px;
                    padding: 0 16px;
                    background: {DARK.glass_bg};
                    backdrop-filter: blur(24px);
                    -webkit-backdrop-filter: blur(24px);
                    border-bottom: 1px solid {DARK.glass_border};
                    flex-shrink: 0;
                    z-index: 10;
                ",

                // Back button
                button {
                    style: "
                        width: 36px;
                        height: 36px;
                        border-radius: 12px;
                        background: {DARK.bg_tertiary};
                        border: 1px solid {DARK.border_subtle};
                        color: {DARK.text_primary};
                        font-size: 18px;
                        cursor: pointer;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        transition: all 200ms cubic-bezier(.22,1,.36,1);
                    ",
                    onclick: move |_| { nav.push(Route::Home {}); },
                    "←"
                }

                // Avatar
                Avatar { pubkey: contact.public_key.clone(), size: 40 }

                // Name + status
                div {
                    style: "flex: 1; min-width: 0;",

                    div {
                        style: "
                            font-size: 16px;
                            font-weight: 700;
                            color: {DARK.text_primary};
                            white-space: nowrap;
                            overflow: hidden;
                            text-overflow: ellipsis;
                        ",
                        "{display_name}"
                    }

                    div {
                        style: "
                            font-size: 12px;
                            color: {DARK.accent_green};
                            display: flex;
                            align-items: center;
                            gap: 5px;
                        ",
                        span {
                            style: "
                                width: 7px;
                                height: 7px;
                                border-radius: 50%;
                                background: {DARK.accent_green};
                                display: inline-block;
                                box-shadow: 0 0 8px {DARK.accent_green}88;
                            ",
                        }
                        "Encrypted"
                    }
                }

                // Menu button
                button {
                    style: "
                        width: 36px;
                        height: 36px;
                        border-radius: 12px;
                        background: none;
                        border: 1px solid transparent;
                        color: {DARK.text_secondary};
                        font-size: 18px;
                        cursor: pointer;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        transition: all 200ms ease;
                    ",
                    "⋮"
                }
            }

            // ─── Messages ───────────────────────
            div {
                style: "
                    flex: 1;
                    overflow-y: auto;
                    padding: 16px;
                    display: flex;
                    flex-direction: column;
                    gap: 4px;
                ",

                if messages.is_empty() {
                    // ─── Empty Chat State ─────────
                    div {
                        style: "
                            flex: 1;
                            display: flex;
                            flex-direction: column;
                            align-items: center;
                            justify-content: center;
                            gap: 12px;
                            animation: {presets::fade_in_up()};
                        ",

                        // Lock icon with glass backing
                        div {
                            style: "
                                width: 56px;
                                height: 56px;
                                border-radius: 50%;
                                background: {DARK.glass_bg};
                                backdrop-filter: blur(12px);
                                -webkit-backdrop-filter: blur(12px);
                                border: 1px solid {DARK.glass_border};
                                display: flex;
                                align-items: center;
                                justify-content: center;
                                font-size: 24px;
                                margin-bottom: 4px;
                                box-shadow: {DARK.shadow_md};
                            ",
                            "🔒"
                        }

                        p {
                            style: "
                                font-size: 15px;
                                font-weight: 600;
                                color: {DARK.text_primary};
                                text-align: center;
                            ",
                            "End-to-end encrypted"
                        }
                        p {
                            style: "
                                font-size: 13px;
                                color: {DARK.text_tertiary};
                                text-align: center;
                                max-width: 240px;
                                line-height: 1.5;
                            ",
                            "Send a message to start the conversation. Only you and this contact can read them."
                        }
                    }
                } else {
                    for (i, msg) in messages.iter().enumerate() {
                        MessageBubble {
                            key: "{msg.id}",
                            content: msg.content.clone(),
                            timestamp: msg.timestamp.format("%H:%M").to_string(),
                            is_sent: msg.sender_key == our_key,
                            status: msg.status.clone(),
                            index: i as u32,
                        }
                    }
                }
            }

            // ─── Input Bar ──────────────────────
            InputBar {
                value: message_input.read().clone(),
                on_input: move |val: String| message_input.set(val),
                on_send: {
                    let contact_id = contact_id.clone();
                    move |_| {
                        let msg = message_input.read().trim().to_string();
                        if !msg.is_empty() {
                            let mut state = state;
                            actions::action_send_message(&mut state, &contact_id, &msg);
                            message_input.set(String::new());
                        }
                    }
                },
            }
        }
    }
}
