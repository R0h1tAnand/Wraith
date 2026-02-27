use dioxus::prelude::*;
use crate::core::types::TorStatus;
use crate::state::app_state::AppState;
use crate::ui::theme::DARK;
use crate::ui::animations::presets;

/// Tor connection status pill displayed at the top of relevant screens.
#[component]
pub fn StatusBar() -> Element {
    let state = use_context::<Signal<AppState>>();
    let status = &state.read().tor_status;
    let color = status.color();
    let label = status.label();

    let bg_alpha = "26"; // ~15% opacity in hex
    let bg_color = format!("{}{}",color, bg_alpha);

    let animation = match status {
        TorStatus::Connecting => presets::pulse_glow(),
        TorStatus::Connected => String::new(),
        TorStatus::Degraded => presets::pulse_glow(),
        TorStatus::Disconnected => String::new(),
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
                    background: {bg_color};
                    font-size: 13px;
                    font-weight: 500;
                    color: {color};
                    animation: {animation};
                ",

                // Status dot
                div {
                    style: "
                        position: relative;
                        width: 8px;
                        height: 8px;
                    ",

                    // Static dot
                    div {
                        style: "
                            width: 8px;
                            height: 8px;
                            border-radius: 50%;
                            background: {color};
                        ",
                    }

                    // Ping animation for connected state
                    if *status == TorStatus::Connected {
                        div {
                            style: "
                                position: absolute;
                                top: 0;
                                left: 0;
                                width: 8px;
                                height: 8px;
                                border-radius: 50%;
                                background: {color};
                                animation: ping 1.5s cubic-bezier(0, 0, 0.2, 1) infinite;
                            ",
                        }
                    }

                    // Spinner for connecting state
                    if *status == TorStatus::Connecting {
                        div {
                            style: "
                                position: absolute;
                                top: -4px;
                                left: -4px;
                                width: 16px;
                                height: 16px;
                                border: 2px solid transparent;
                                border-top-color: {color};
                                border-radius: 50%;
                                animation: {presets::spin()};
                            ",
                        }
                    }
                }

                "{label}"
            }
        }
    }
}
