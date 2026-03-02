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
    let state = use_context::<Signal<AppState>>();
    let nav = use_navigator();
    let mut copied = use_signal(|| false);

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
            }
        }
    }
}
