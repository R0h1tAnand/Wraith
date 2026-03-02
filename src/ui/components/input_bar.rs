use dioxus::prelude::*;
use crate::ui::theme::DARK;

/// Message input bar with send button.
#[component]
pub fn InputBar(
    value: String,
    on_input: EventHandler<String>,
    on_send: EventHandler<()>,
) -> Element {
    let can_send = !value.trim().is_empty();
    let text_color = if can_send { "#FFFFFF" } else { DARK.text_tertiary };
    let bg_color = if can_send { DARK.accent_primary } else { DARK.bg_tertiary };
    let cursor_style = if can_send { "pointer" } else { "default" };
    let transform = if can_send { "scale(1)" } else { "scale(0.9)" };

    rsx! {
        div {
            style: "
                display: flex;
                align-items: center;
                gap: 8px;
                padding: 8px 12px;
                padding-bottom: max(8px, env(safe-area-inset-bottom, 8px));
                background: {DARK.bg_secondary};
                border-top: 1px solid {DARK.border_subtle};
                flex-shrink: 0;
            ",

            // Attachment button (future)
            button {
                style: "
                    width: 40px;
                    height: 40px;
                    border-radius: 50%;
                    background: {DARK.bg_tertiary};
                    border: none;
                    color: {DARK.text_secondary};
                    font-size: 18px;
                    cursor: pointer;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    flex-shrink: 0;
                    transition: all 150ms ease;
                ",
                "🔗"
            }

            // Text input
            input {
                class: "input-field",
                style: "
                    flex: 1;
                    border-radius: 9999px;
                    padding: 10px 16px;
                    font-size: 15px;
                ",
                placeholder: "Message...",
                value: "{value}",
                oninput: move |evt| on_input.call(evt.value()),
                onkeypress: move |evt| {
                    if evt.key() == Key::Enter {
                        on_send.call(());
                    }
                },
            }

            // Send button
            button {
                style: "
                    width: 40px;
                    height: 40px;
                    border-radius: 50%;
                    background: {bg_color};
                    border: none;
                    color: {text_color};
                    font-size: 16px;
                    cursor: {cursor_style};
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    flex-shrink: 0;
                    transition: all 150ms ease;
                    transform: {transform};
                ",
                disabled: !can_send,
                onclick: move |_| {
                    if can_send {
                        on_send.call(());
                    }
                },
                "Send"
            }
        }
    }
}
