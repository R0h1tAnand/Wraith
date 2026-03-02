use dioxus::prelude::*;
use crate::app::Route;
use crate::state::app_state::AppState;
use crate::ui::theme::DARK;
use crate::ui::animations::presets;

/// Settings screen.
#[component]
pub fn Settings() -> Element {
    let state = use_context::<Signal<AppState>>();
    let nav = use_navigator();
    let mut show_wipe_confirm = use_signal(|| false);

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
                    padding: 0 20px;
                    flex-shrink: 0;
                ",

                button {
                    style: "
                        background: none;
                        border: none;
                        color: {DARK.text_primary};
                        font-size: 20px;
                        cursor: pointer;
                        padding: 8px;
                    ",
                    onclick: move |_| { nav.push(Route::Home {}); },
                    "←"
                }

                span {
                    style: "
                        flex: 1;
                        text-align: center;
                        font-size: 18px;
                        font-weight: 700;
                        color: {DARK.text_primary};
                    ",
                    "Settings"
                }

                div { style: "width: 36px;" }
            }

            // ─── Content ────────────────────────
            div {
                style: "
                    flex: 1;
                    overflow-y: auto;
                    padding: 8px 16px 80px 16px;
                ",

                // Security section
                SectionHeader { title: "Security" }

                SettingsItem {
                    icon: "🔒",
                    title: "Passphrase Lock",
                    subtitle: "Lock app with PIN or biometrics",
                    has_toggle: true,
                    enabled: false,
                }

                SettingsItem {
                    icon: "🗑️",
                    title: "Auto-clear Messages",
                    subtitle: "Never",
                    has_toggle: false,
                    enabled: false,
                }

                // Network section
                SectionHeader { title: "Network" }

                SettingsItem {
                    icon: "🧅",
                    title: "Tor Status",
                    subtitle: state.read().tor_status.label(),
                    has_toggle: false,
                    enabled: false,
                }

                div {
                    style: "padding: 0 4px; margin-bottom: 8px;",
                    button {
                        class: "btn-secondary",
                        style: "width: 100%;",
                        "🔄 Rebuild Tor Circuit"
                    }
                }

                SettingsItem {
                    icon: "⏱️",
                    title: "Connection Timeout",
                    subtitle: "30 seconds",
                    has_toggle: false,
                    enabled: false,
                }

                // Notifications section
                SectionHeader { title: "Notifications" }

                SettingsItem {
                    icon: "🔔",
                    title: "Push Notifications",
                    subtitle: "Disabled for privacy",
                    has_toggle: true,
                    enabled: false,
                }

                // Danger zone
                SectionHeader { title: "Danger Zone" }

                div {
                    style: "padding: 0 4px; margin-bottom: 24px;",
                    button {
                        class: "btn-danger",
                        onclick: move |_| show_wipe_confirm.set(true),
                        "🗑️ Wipe All Data"
                    }
                }

                // About section
                SectionHeader { title: "About" }

                div {
                    style: "
                        background: {DARK.bg_secondary};
                        border: 1px solid {DARK.border_subtle};
                        border-radius: 16px;
                        padding: 16px;
                    ",

                    p {
                        style: "
                            font-size: 14px;
                            color: {DARK.text_primary};
                            margin-bottom: 4px;
                        ",
                        "Wraith v0.1.0"
                    }
                    p {
                        style: "
                            font-size: 13px;
                            color: {DARK.text_secondary};
                        ",
                        "Built with Rust 🦀 + Dioxus"
                    }
                }
            }

            // ─── Wipe Confirmation Modal ────────
            if *show_wipe_confirm.read() {
                div {
                    style: "
                        position: fixed;
                        top: 0;
                        left: 0;
                        right: 0;
                        bottom: 0;
                        background: rgba(0, 0, 0, 0.7);
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        z-index: 200;
                        padding: 32px;
                    ",

                    div {
                        style: "
                            background: {DARK.bg_secondary};
                            border: 1px solid {DARK.border_subtle};
                            border-radius: 24px;
                            padding: 24px;
                            width: 100%;
                            max-width: 340px;
                            animation: {presets::scale_in()};
                        ",

                        h3 {
                            style: "
                                font-size: 18px;
                                font-weight: 700;
                                color: {DARK.text_primary};
                                margin-bottom: 8px;
                            ",
                            "Wipe All Data?"
                        }

                        p {
                            style: "
                                font-size: 14px;
                                color: {DARK.text_secondary};
                                margin-bottom: 24px;
                                line-height: 1.5;
                            ",
                            "This will permanently delete your identity, contacts, and all messages. This cannot be undone."
                        }

                        div {
                            style: "display: flex; gap: 12px;",

                            button {
                                class: "btn-secondary",
                                style: "flex: 1;",
                                onclick: move |_| show_wipe_confirm.set(false),
                                "Cancel"
                            }

                            button {
                                class: "btn-danger",
                                style: "flex: 1;",
                                onclick: move |_| {
                                    show_wipe_confirm.set(false);
                                    // TODO: Actually wipe storage
                                },
                                "Wipe Everything"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SectionHeader(title: &'static str) -> Element {
    rsx! {
        div {
            style: "
                padding: 16px 4px 8px 4px;
            ",
            span {
                style: "
                    font-size: 12px;
                    font-weight: 700;
                    color: {DARK.text_secondary};
                    text-transform: uppercase;
                    letter-spacing: 0.08em;
                ",
                "{title}"
            }
        }
    }
}

#[component]
fn SettingsItem(
    icon: &'static str,
    title: &'static str,
    subtitle: &'static str,
    has_toggle: bool,
    enabled: bool,
) -> Element {
    let bg_color = if enabled { DARK.accent_primary } else { DARK.bg_tertiary };
    let toggle_left = if enabled { "22px" } else { "2px" };

    rsx! {
        div {
            style: "
                display: flex;
                align-items: center;
                gap: 12px;
                padding: 14px;
                background: {DARK.bg_secondary};
                border: 1px solid {DARK.border_subtle};
                border-radius: 16px;
                margin-bottom: 8px;
                cursor: pointer;
                transition: background 150ms ease;
            ",

            span { style: "font-size: 20px; flex-shrink: 0;", "{icon}" }

            div {
                style: "flex: 1; min-width: 0;",

                div {
                    style: "
                        font-size: 15px;
                        font-weight: 500;
                        color: {DARK.text_primary};
                    ",
                    "{title}"
                }

                div {
                    style: "
                        font-size: 13px;
                        color: {DARK.text_secondary};
                        margin-top: 2px;
                    ",
                    "{subtitle}"
                }
            }

            if has_toggle {
                // Toggle switch
                div {
                    style: "
                        width: 44px;
                        height: 24px;
                        border-radius: 12px;
                        background: {bg_color};
                        position: relative;
                        transition: background 200ms ease;
                        flex-shrink: 0;
                    ",
                    div {
                        style: "
                            width: 20px;
                            height: 20px;
                            border-radius: 50%;
                            background: white;
                            position: absolute;
                            top: 2px;
                            left: {toggle_left};
                            transition: left 200ms ease;
                        ",
                    }
                }
            } else {
                span {
                    style: "
                        color: {DARK.text_tertiary};
                        font-size: 16px;
                        flex-shrink: 0;
                    ",
                    "›"
                }
            }
        }
    }
}
