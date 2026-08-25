use dioxus::prelude::*;
use crate::ui::theme::DARK;
use crate::ui::animations::presets;

/// Message delivery status.
#[derive(Clone, PartialEq)]
pub enum MessageStatus {
    Sending,
    Sent,
    Delivered,
    Read,
    Failed,
}

/// Whether this bubble was sent by the local user or received.
#[derive(Clone, PartialEq)]
pub enum BubbleDirection {
    Sent,
    Received,
}

/// A single chat message bubble.
///
/// Sent messages use an accent gradient fill.
/// Received messages use a frosted glass surface.
#[component]
pub fn MessageBubble(
    /// The message text.
    text: String,
    /// Time string (e.g. "14:32").
    time: String,
    /// Direction of the message.
    direction: BubbleDirection,
    /// Delivery status (only relevant for sent).
    #[props(default = MessageStatus::Sent)]
    status: MessageStatus,
    /// Position for stagger animation.
    #[props(default = 0)]
    index: u32,
) -> Element {
    let is_sent = direction == BubbleDirection::Sent;
    let anim = presets::stagger(index, 20);

    let (bg, border, text_color, align, radius) = if is_sent {
        (
            DARK.gradient_accent,
            "transparent",
            DARK.text_on_accent,
            "flex-end",
            "20px 20px 6px 20px",
        )
    } else {
        (
            DARK.glass_bg_heavy.to_string().leak() as &'static str,
            DARK.glass_border,
            DARK.text_primary,
            "flex-start",
            "20px 20px 20px 6px",
        )
    };

    let shadow = if is_sent {
        "0 4px 16px rgba(124, 106, 247, 0.2)"
    } else {
        DARK.shadow_soft
    };

    let backdrop = if is_sent { "none" } else { DARK.glass_backdrop };

    rsx! {
        div {
            style: "
                display: flex;
                justify-content: {align};
                padding: 2px 16px;
                animation: {anim};
            ",

            div {
                style: "
                    max-width: 78%;
                    padding: 10px 14px 8px;
                    border-radius: {radius};
                    background: {bg};
                    border: 1px solid {border};
                    box-shadow: {shadow};
                    backdrop-filter: {backdrop};
                    -webkit-backdrop-filter: {backdrop};
                ",

                // Message text
                p {
                    style: "
                        font-size: 14.5px;
                        line-height: 1.45;
                        color: {text_color};
                        word-wrap: break-word;
                        letter-spacing: -0.01em;
                    ",
                    "{text}"
                }

                // Footer: time + status
                div {
                    style: "
                        display: flex;
                        align-items: center;
                        justify-content: flex-end;
                        gap: 5px;
                        margin-top: 4px;
                    ",

                    span {
                        style: "
                            font-size: 11px;
                            color: {time_color};
                            opacity: 0.7;
                        ",
                        time_color = if is_sent { DARK.text_on_accent } else { DARK.text_tertiary },
                        "{time}"
                    }

                    if is_sent {
                        { render_status(&status) }
                    }
                }
            }
        }
    }
}

fn render_status(status: &MessageStatus) -> Element {
    let (icon, color) = match status {
        MessageStatus::Sending  => ("↑", "rgba(255,255,255,0.4)"),
        MessageStatus::Sent     => ("✓", "rgba(255,255,255,0.6)"),
        MessageStatus::Delivered => ("✓✓", "rgba(255,255,255,0.7)"),
        MessageStatus::Read     => ("✓✓", DARK.accent_cyan),
        MessageStatus::Failed   => ("✕", DARK.accent_red),
    };

    rsx! {
        span {
            style: "
                font-size: 11px;
                color: {color};
                font-weight: 600;
                letter-spacing: -1px;
            ",
            "{icon}"
        }
    }
}
