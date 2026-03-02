use dioxus::prelude::*;
use crate::app::Route;
use crate::state::app_state::AppState;
use crate::state::actions;
use crate::ui::theme::DARK;
use crate::ui::animations::presets;

/// Add contact / start new conversation screen.
#[component]
pub fn NewChat() -> Element {
    let mut state = use_context::<Signal<AppState>>();
    let nav = use_navigator();
    let mut public_key = use_signal(|| String::new());
    let mut nickname = use_signal(|| String::new());
    let mut tab = use_signal(|| 0u32); // 0 = Paste Key, 1 = Scan QR

    let is_valid_key = {
        let k = public_key.read();
        let raw = k.strip_prefix("wraith:").unwrap_or(&k);
        raw.len() == 64 && raw.chars().all(|c| c.is_ascii_hexdigit())
    };

    rsx! {
        div {
            style: "
                width: 100%;
                height: 100vh;
                display: flex;
                flex-direction: column;
                background: {DARK.bg_primary};
                animation: {presets::fade_in_up()};
            ",

            // ─── Header ─────────────────────────
            div {
                style: "
                    height: 60px;
                    display: flex;
                    align-items: center;
                    justify-content: space-between;
                    padding: 0 20px;
                    flex-shrink: 0;
                ",

                h2 {
                    style: "
                        font-size: 18px;
                        font-weight: 700;
                        color: {DARK.text_primary};
                    ",
                    "New Conversation"
                }

                button {
                    style: "
                        background: none;
                        border: none;
                        color: {DARK.text_secondary};
                        font-size: 20px;
                        cursor: pointer;
                        padding: 8px;
                    ",
                    onclick: move |_| { nav.push(Route::Home {}); },
                    "✕"
                }
            }

            // ─── Tabs ───────────────────────────
            div {
                style: "
                    display: flex;
                    padding: 0 16px;
                    gap: 4px;
                    margin-bottom: 24px;
                    flex-shrink: 0;
                ",

                TabButton {
                    label: "Paste Key",
                    active: *tab.read() == 0,
                    on_click: move |_| tab.set(0),
                }
                TabButton {
                    label: "Scan QR",
                    active: *tab.read() == 1,
                    on_click: move |_| tab.set(1),
                }
            }

            // ─── Content ────────────────────────
            div {
                style: "
                    flex: 1;
                    padding: 0 16px;
                    overflow-y: auto;
                ",

                if *tab.read() == 0 {
                    // Paste Key tab
                    div {
                        style: "animation: {presets::fade_in_up()};",

                        // Public key input
                        div {
                            style: "margin-bottom: 16px;",

                            label {
                                style: "
                                    display: block;
                                    font-size: 13px;
                                    font-weight: 600;
                                    color: {DARK.text_secondary};
                                    margin-bottom: 8px;
                                ",
                                "Public Key"
                            }

                            div {
                                style: "position: relative;",

                                textarea {
                                    class: "input-field",
                                    style: "
                                        font-family: 'Cascadia Code', 'Fira Code', monospace;
                                        font-size: 13px;
                                        min-height: 80px;
                                        resize: none;
                                    ",
                                    placeholder: "Enter public key (wraith:abc123...)",
                                    value: "{public_key}",
                                    oninput: move |evt| public_key.set(evt.value()),
                                }

                                // Validation indicator
                                if !public_key.read().is_empty() {
                                    div {
                                        style: "
                                            position: absolute;
                                            top: 12px;
                                            right: 12px;
                                            font-size: 16px;
                                        ",
                                        if is_valid_key { "✅" } else { "❌" }
                                    }
                                }
                            }
                        }

                        // Nickname input
                        div {
                            style: "margin-bottom: 24px;",

                            label {
                                style: "
                                    display: block;
                                    font-size: 13px;
                                    font-weight: 600;
                                    color: {DARK.text_secondary};
                                    margin-bottom: 8px;
                                ",
                                "Nickname (optional)"
                            }

                            input {
                                class: "input-field",
                                placeholder: "e.g. Alice",
                                value: "{nickname}",
                                oninput: move |evt| nickname.set(evt.value()),
                            }
                        }

                        // Start conversation button
                        button {
                            class: "btn-primary",
                            disabled: !is_valid_key,
                            onclick: move |_| {
                                let key = public_key.read().clone();
                                let raw_key = key.strip_prefix("wraith:").unwrap_or(&key).to_string();
                                let nick = {
                                    let n = nickname.read().trim().to_string();
                                    if n.is_empty() { None } else { Some(n) }
                                };
                                actions::action_add_contact(&mut state, raw_key, nick);
                                nav.push(Route::Home {});
                            },
                            "Start Conversation"
                        }
                    }
                } else {
                    // Scan QR tab (placeholder)
                    div {
                        style: "
                            display: flex;
                            flex-direction: column;
                            align-items: center;
                            justify-content: center;
                            padding: 48px;
                            animation: {presets::fade_in_up()};
                        ",

                        // Camera viewfinder
                        div {
                            style: "
                                width: 240px;
                                height: 240px;
                                border: 2px solid {DARK.border_subtle};
                                border-radius: 24px;
                                display: flex;
                                align-items: center;
                                justify-content: center;
                                position: relative;
                                margin-bottom: 24px;
                            ",

                            // Corner brackets
                            div { style: "position: absolute; top: -2px; left: -2px; width: 24px; height: 24px; border-top: 3px solid {DARK.accent_primary}; border-left: 3px solid {DARK.accent_primary}; border-radius: 8px 0 0 0;" }
                            div { style: "position: absolute; top: -2px; right: -2px; width: 24px; height: 24px; border-top: 3px solid {DARK.accent_primary}; border-right: 3px solid {DARK.accent_primary}; border-radius: 0 8px 0 0;" }
                            div { style: "position: absolute; bottom: -2px; left: -2px; width: 24px; height: 24px; border-bottom: 3px solid {DARK.accent_primary}; border-left: 3px solid {DARK.accent_primary}; border-radius: 0 0 0 8px;" }
                            div { style: "position: absolute; bottom: -2px; right: -2px; width: 24px; height: 24px; border-bottom: 3px solid {DARK.accent_primary}; border-right: 3px solid {DARK.accent_primary}; border-radius: 0 0 8px 0;" }

                            span {
                                style: "
                                    font-size: 48px;
                                    opacity: 0.3;
                                ",
                                "📷"
                            }
                        }

                        p {
                            style: "
                                font-size: 14px;
                                color: {DARK.text_secondary};
                                animation: {presets::pulse_glow()};
                            ",
                            "Camera not available on desktop"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TabButton(label: &'static str, active: bool, on_click: EventHandler<()>) -> Element {
    let bg = if active { DARK.accent_primary } else { DARK.bg_tertiary };
    let color = if active { DARK.text_primary } else { DARK.text_secondary };

    rsx! {
        button {
            style: "
                flex: 1;
                padding: 10px 16px;
                background: {bg};
                color: {color};
                border: none;
                border-radius: 9999px;
                font-family: 'Inter', sans-serif;
                font-size: 14px;
                font-weight: 600;
                cursor: pointer;
                transition: all 150ms ease;
            ",
            onclick: move |_| on_click.call(()),
            "{label}"
        }
    }
}
