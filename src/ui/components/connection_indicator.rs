use dioxus::prelude::*;
use crate::ui::theme::DARK;

/// Network connection states.
#[derive(Clone, PartialEq)]
pub enum ConnectionState {
    Connected,
    Connecting,
    Disconnected,
}

/// Compact connection status indicator with colored dot and label.
///
/// Uses the `breathe` animation for the Connecting state.
#[component]
pub fn ConnectionIndicator(
    /// Current connection state.
    state: ConnectionState,
    /// Whether to show the text label alongside the dot.
    #[props(default = true)]
    show_label: bool,
) -> Element {
    let (color, label, anim) = match state {
        ConnectionState::Connected => (
            DARK.accent_green,
            "Connected",
            "none",
        ),
        ConnectionState::Connecting => (
            DARK.accent_amber,
            "Connecting…",
            "breathe 1.5s ease-in-out infinite",
        ),
        ConnectionState::Disconnected => (
            DARK.accent_red,
            "Disconnected",
            "none",
        ),
    };

    rsx! {
        div {
            style: "
                display: flex;
                align-items: center;
                gap: 6px;
            ",

            // Dot with glow
            div {
                style: "
                    width: 8px;
                    height: 8px;
                    border-radius: 50%;
                    background: {color};
                    box-shadow: 0 0 8px {color};
                    animation: {anim};
                    flex-shrink: 0;
                ",
            }

            if show_label {
                span {
                    style: "
                        font-size: 12px;
                        color: {text_sec};
                        font-weight: 500;
                    ",
                    text_sec = DARK.text_secondary,
                    "{label}"
                }
            }
        }
    }
}
