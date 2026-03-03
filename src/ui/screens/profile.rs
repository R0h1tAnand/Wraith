use dioxus::prelude::*;
use crate::app::Route;
use crate::state::app_state::AppState;
use crate::ui::theme::DARK;
use crate::ui::animations::presets;
use crate::ui::components::avatar::Avatar;
use crate::ui::components::qr_code::QrCode;

/// Profile screen — view and share your identity.
#[component]
pub fn Profile() -> Element {
    let mut state = use_context::<Signal<AppState>>();
    let nav = use_navigator();
    let mut copied = use_signal(|| false);
    let mut backup_copied = use_signal(|| false);
    let mut show_reset_modal = use_signal(|| false);
    let mut backup_data = use_signal(|| String::new());

    let (public_key, short_key) = {
        let s = state.read();
        match &s.identity {
            Some(id) => (id.public_key_hex(), id.public_key_short()),
            None => ("No identity".to_string(), "No identity".to_string()),
        }
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
                    "Your Profile"
                }

                // Spacer for centering
                div { style: "width: 36px;" }
            }

            // ─── Content ────────────────────────
            div {
                style: "
                    flex: 1;
                    overflow-y: auto;
                    padding: 24px 16px;
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                ",

                // Large avatar
                div {
                    style: "margin-bottom: 16px;",
                    Avatar { pubkey: public_key.replace("wraith:", ""), size: 120 }
                }

                // Display name
                h2 {
                    style: "
                        font-size: 22px;
                        font-weight: 700;
                        color: {DARK.text_primary};
                        margin-bottom: 4px;
                    ",
                    "Anonymous User"
                }

                p {
                    style: "
                        font-size: 13px;
                        color: {DARK.text_secondary};
                        margin-bottom: 24px;
                    ",
                    "{short_key}"
                }

                // Key card
                div {
                    style: "
                        width: 100%;
                        background: {DARK.bg_secondary};
                        border: 1px solid {DARK.border_subtle};
                        border-radius: 16px;
                        padding: 16px;
                        margin-bottom: 16px;
                    ",

                    p {
                        style: "
                            font-size: 11px;
                            font-weight: 600;
                            color: {DARK.text_secondary};
                            text-transform: uppercase;
                            letter-spacing: 0.05em;
                            margin-bottom: 8px;
                        ",
                        "Your Public Key"
                    }

                    p {
                        style: "
                            font-family: 'Cascadia Code', 'Fira Code', monospace;
                            font-size: 11px;
                            color: {DARK.accent_primary};
                            word-break: break-all;
                            line-height: 1.6;
                        ",
                        "{public_key}"
                    }
                }

                // Action buttons
                div {
                    style: "
                        width: 100%;
                        display: flex;
                        gap: 12px;
                        margin-bottom: 32px;
                    ",

                    button {
                        class: "btn-secondary",
                        style: "flex: 1;",
                        onclick: move |_| copied.set(true),
                        if *copied.read() { "✓ Copied!" } else { "📋 Copy Key" }
                    }

                    button {
                        class: "btn-secondary",
                        style: "flex: 1;",
                        "📤 Share"
                    }
                }

                // QR Code
                div {
                    style: "
                        width: 100%;
                        display: flex;
                        flex-direction: column;
                        align-items: center;
                        padding: 24px;
                        background: {DARK.bg_secondary};
                        border: 1px solid {DARK.border_subtle};
                        border-radius: 16px;
                        margin-bottom: 32px;
                    ",

                    QrCode { data: public_key.clone(), size: 200 }

                    p {
                        style: "
                            margin-top: 16px;
                            font-size: 13px;
                            color: {DARK.text_secondary};
                            text-align: center;
                        ",
                        "Others can scan this to message you"
                    }
                }

                // Danger Zone - Reset Profile
                div {
                    style: "
                        width: 100%;
                        display: flex;
                        flex-direction: column;
                        align-items: center;
                        padding: 24px;
                        background: rgba(248, 113, 113, 0.05);
                        border: 1px solid rgba(248, 113, 113, 0.2);
                        border-radius: 16px;
                        text-align: center;
                    ",
                    h3 {
                        style: "
                            color: #F87171;
                            font-size: 16px;
                            margin-bottom: 8px;
                            font-weight: 600;
                        ",
                        "Danger Zone"
                    }
                    p {
                        style: "
                            font-size: 13px;
                            color: {DARK.text_secondary};
                            margin-bottom: 16px;
                            max-width: 250px;
                        ",
                        "Permanently delete your identity, contacts, and all messages from this device."
                    }
                    
                    button {
                        style: "
                            background: rgba(248, 113, 113, 0.1);
                            color: #F87171;
                            border: 1px solid rgba(248, 113, 113, 0.3);
                            padding: 12px 24px;
                            border-radius: 12px;
                            font-weight: 600;
                            font-size: 14px;
                            cursor: pointer;
                            transition: all 0.2s;
                        ",
                        onclick: move |_| {
                            let backup = crate::state::actions::action_backup_chats(&state);
                            backup_data.set(backup);
                            show_reset_modal.set(true);
                        },
                        "Reset Profile"
                    }
                }
                
                // Reset Modal
                if *show_reset_modal.read() {
                    div {
                        style: "
                            position: fixed;
                            top: 0; left: 0; right: 0; bottom: 0;
                            background: rgba(0, 0, 0, 0.8);
                            backdrop-filter: blur(8px);
                            display: flex;
                            align-items: center;
                            justify-content: center;
                            padding: 24px;
                            z-index: 100;
                        ",
                        div {
                            style: "
                                background: {DARK.bg_primary};
                                border: 1px solid {DARK.border_subtle};
                                border-radius: 20px;
                                padding: 24px;
                                width: 100%;
                                max-width: 400px;
                                display: flex;
                                flex-direction: column;
                                animation: {presets::scale_in()};
                            ",
                            h3 { style: "color: #F87171; margin-bottom: 12px; font-size: 20px; text-align: center;", "Final Warning" }
                            p {
                                style: "color: {DARK.text_secondary}; font-size: 14px; margin-bottom: 24px; text-align: center; line-height: 1.5;",
                                "You are about to permanently delete your Wraith identity. Your contacts and messages will be wiped."
                            }
                            
                            p { style: "color: {DARK.text_primary}; font-size: 12px; font-weight: 600; margin-bottom: 8px;", "Chat Backup (Save this!)" }
                            textarea {
                                readonly: true,
                                style: "
                                    width: 100%;
                                    height: 120px;
                                    background: {DARK.bg_secondary};
                                    border: 1px solid {DARK.border_subtle};
                                    border-radius: 8px;
                                    padding: 12px;
                                    color: {DARK.text_secondary};
                                    font-family: monospace;
                                    font-size: 11px;
                                    resize: none;
                                    margin-bottom: 16px;
                                ",
                                "{backup_data}"
                            }
                            
                            button {
                                class: "btn-secondary",
                                style: "margin-bottom: 24px;",
                                onclick: move |_| backup_copied.set(true),
                                if *backup_copied.read() { "✓ Backup Copied to Clipboard" } else { "📋 Copy JSON Backup" }
                            }
                            
                            div {
                                style: "display: flex; gap: 12px;",
                                button {
                                    style: "flex: 1; padding: 12px; border-radius: 12px; border: 1px solid {DARK.border_subtle}; background: transparent; color: {DARK.text_primary}; cursor: pointer;",
                                    onclick: move |_| show_reset_modal.set(false),
                                    "Cancel"
                                }
                                button {
                                    style: "flex: 1; padding: 12px; border-radius: 12px; border: none; background: #DC2626; color: white; cursor: pointer; font-weight: 600;",
                                    onclick: move |_| {
                                        crate::state::actions::action_reset_profile(&mut state);
                                        nav.replace(crate::app::Route::Onboarding {});
                                    },
                                    "Delete Everything"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
