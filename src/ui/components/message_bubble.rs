use dioxus::prelude::*;
use crate::core::types::MessageStatus;
use crate::ui::theme::DARK;
use crate::ui::animations::presets;

/// Individual message bubble.
#[component]
pub fn MessageBubble(
    content: String,
    timestamp: String,
    is_sent: bool,
    status: MessageStatus,
    index: u32,
) -> Element {
    let (bg, align, radius) = if is_sent {
        (DARK.accent_primary, "flex-end", "20px 20px 4px 20px")
    } else {
        (DARK.bg_tertiary, "flex-start", "20px 20px 20px 4px")
    };

    let text_color = if is_sent { "#FFFFFF" } else { DARK.text_primary };

    let status_icon = match status {
        MessageStatus::Sending => "◌",
        MessageStatus::Sent => "✓",
        MessageStatus::Delivered => "✓✓",
        MessageStatus::Read => "✓✓",
        MessageStatus::Failed => "✕",
    };

    let status_color = match status {
        MessageStatus::Read => DARK.accent_primary,
        MessageStatus::Failed => DARK.accent_red,
        _ => if is_sent { "rgba(255,255,255,0.5)" } else { DARK.text_tertiary },
    };

    let delay = index * 30;

    rsx! {
        div {
            style: "
                display: flex;
                justify-content: {align};
                padding: 2px 0;
                animation: {presets::fade_in_up_delayed(delay)};
            ",

            div {
                style: "
                    max-width: 75%;
                    background: {bg};
                    border-radius: {radius};
                    padding: 10px 14px;
                    position: relative;
                ",

                // Message content
                p {
                    style: "
                        font-size: 15px;
                        color: {text_color};
                        line-height: 1.4;
                        word-break: break-word;
                    ",
                    "{content}"
                }

                // Timestamp + status
                div {
                    style: "
                        display: flex;
                        align-items: center;
                        justify-content: flex-end;
                        gap: 4px;
                        margin-top: 4px;
                    ",

                    span {
                        style: "
                            font-size: 11px;
                            color: {if is_sent { "rgba(255,255,255,0.5)" } else { DARK.text_tertiary }};
                        ",
                        "{timestamp}"
                    }

                    if is_sent {
                        span {
                            style: "
                                font-size: 11px;
                                color: {status_color};
                            ",
                            "{status_icon}"
                        }
                    }
                }
            }
        }
    }
}
