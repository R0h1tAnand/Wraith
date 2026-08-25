use dioxus::prelude::*;
use crate::core::types::TorStatus;
use crate::state::app_state::AppState;
use crate::ui::theme::DARK;
use crate::ui::animations::presets;

/// Glassmorphic Tor status pill with refined animations.
#[component]
pub fn StatusBar() -> Element {
    let state = use_context::<Signal<AppState>>();
    let status = &state.read().tor_status;
    let label = status.label();

    let (gradient, dot_color, glow) = match status {
        TorStatus::Connected => (
            DARK.gradient_success,
            DARK.accent_green,
            format!("0 0 10px {}", DARK.accent_green),
        ),
        TorStatus::Connecting => (
            DARK.gradient_warning,
            DARK.accent_amber,
            format!("0 0 10px {}", DARK.accent_amber),
        ),
        TorStatus::Degraded => (
            DARK.gradient_warning,
            DARK.accent_amber,
            format!("0 0 10px {}", DARK.accent_amber),
        ),
        TorStatus::Disconnected => (
            DARK.gradient_error,
            DARK.accent_red,
            format!("0 0 10px {}", DARK.accent_red),
        ),
    };

    let animation = match status {
        TorStatus::Connecting => presets::pulse_glow(),
        TorStatus::Degraded   => presets::pulse_glow(),
        _ => String::new(),
    };

    rsx! {
        div {
            style: "
                display: flex;
                justify-content: center;
                padding: 8px 16px;
                flex-shrink: 0;
            ",

            div {
                style: "
                    display: inline-flex;
                    align-items: center;
                    gap: 8px;
                    padding: 6px 16px;
                    border-radius: 9999px;
                    background: {glass_bg};
                    border: 1px solid {glass_border};
                    backdrop-filter: {backdrop};
                    -webkit-backdrop-filter: {backdrop};
                    font-size: 13px;
                    font-weight: 600;
                    animation: {animation};
                ",
                glass_bg = DARK.glass_bg,
                glass_border = DARK.glass_border,
                backdrop = DARK.glass_backdrop,

                // Status dot with glow
                div {
                    style: "
                        position: relative;
                        width: 8px;
                        height: 8px;
                    ",

                    div {
                        style: "
                            width: 8px;
                            height: 8px;
                            border-radius: 50%;
                            background: {dot_color};
                            box-shadow: {glow};
                        ",
                    }

                    // Ping for connected
                    if *status == TorStatus::Connected {
                        div {
                            style: "
                                position: absolute;
                                top: 0;
                                left: 0;
                                width: 8px;
                                height: 8px;
                                border-radius: 50%;
                                background: {dot_color};
                                animation: ping 1.5s cubic-bezier(0, 0, 0.2, 1) infinite;
                            ",
                        }
                    }

                    // Spinner for connecting
                    if *status == TorStatus::Connecting {
                        div {
                            style: "
                                position: absolute;
                                top: -4px;
                                left: -4px;
                                width: 16px;
                                height: 16px;
                                border: 2px solid transparent;
                                border-top-color: {dot_color};
                                border-radius: 50%;
                                animation: {spin};
                            ",
                            spin = presets::spin(),
                        }
                    }
                }

                // Gradient label
                span {
                    style: "
                        background: {gradient};
                        -webkit-background-clip: text;
                        -webkit-text-fill-color: transparent;
                        background-clip: text;
                    ",
                    "{label}"
                }
            }
        }
    }
}
