use dioxus::prelude::*;
use crate::core::types::Contact;
use crate::ui::theme::DARK;
use crate::ui::animations::presets;
use crate::ui::components::avatar::Avatar;

/// A single row in the chat list.
#[component]
pub fn ChatListItem(
    contact: Contact,
    delay_ms: u32,
    on_click: EventHandler<()>,
) -> Element {
    let display_name = contact
        .nickname
        .as_deref()
        .unwrap_or_else(|| {
            if contact.public_key.len() > 12 {
                // Will show truncated key
                "Unknown"
            } else {
                &contact.public_key
            }
        });

    let preview = contact
        .last_message_preview
        .as_deref()
        .unwrap_or("No messages yet");

    let time_str = contact
        .last_message_time
        .map(|t| t.format("%H:%M").to_string())
        .unwrap_or_default();

    rsx! {
        div {
            style: "
                display: flex;
                align-items: center;
                gap: 12px;
                padding: 12px 16px;
                cursor: pointer;
                transition: background 150ms ease;
                animation: {presets::fade_in_up_delayed(delay_ms)};
            ",
            // Hover effect via onmouseenter is not great in Dioxus,
            // so we rely on the CSS transition for visual feedback
            onclick: move |_| on_click.call(()),

            // Avatar
            Avatar { pubkey: contact.public_key.clone(), size: 52 }

            // Text content
            div {
                style: "flex: 1; min-width: 0; overflow: hidden;",

                div {
                    style: "
                        display: flex;
                        align-items: center;
                        justify-content: space-between;
                        margin-bottom: 4px;
                    ",

                    span {
                        style: "
                            font-size: 15px;
                            font-weight: 600;
                            color: {DARK.text_primary};
                            overflow: hidden;
                            text-overflow: ellipsis;
                            white-space: nowrap;
                        ",
                        "{display_name}"
                    }

                    span {
                        style: "
                            font-size: 12px;
                            color: {DARK.text_tertiary};
                            flex-shrink: 0;
                            margin-left: 8px;
                        ",
                        "{time_str}"
                    }
                }

                div {
                    style: "
                        display: flex;
                        align-items: center;
                        justify-content: space-between;
                    ",

                    p {
                        style: "
                            font-size: 14px;
                            color: {DARK.text_secondary};
                            overflow: hidden;
                            text-overflow: ellipsis;
                            white-space: nowrap;
                            flex: 1;
                        ",
                        "{preview}"
                    }

                    // Unread badge
                    if contact.unread_count > 0 {
                        div {
                            style: "
                                min-width: 20px;
                                height: 20px;
                                border-radius: 10px;
                                background: {DARK.accent_primary};
                                color: white;
                                font-size: 11px;
                                font-weight: 700;
                                display: flex;
                                align-items: center;
                                justify-content: center;
                                padding: 0 6px;
                                margin-left: 8px;
                                flex-shrink: 0;
                            ",
                            "{contact.unread_count}"
                        }
                    }
                }
            }
        }
    }
}
