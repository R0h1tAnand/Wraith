use dioxus::prelude::*;
use crate::app::Route;
use crate::state::app_state::AppState;
use crate::ui::theme::DARK;
use crate::ui::animations::presets;
use crate::ui::components::chat_list_item::ChatListItem;
use crate::ui::components::status_bar::StatusBar;

/// Home screen — chat list / inbox.
#[component]
pub fn Home() -> Element {
    let state = use_context::<Signal<AppState>>();
    let nav = use_navigator();
    let mut search = use_signal(|| String::new());

    let contacts = {
        let s = state.read();
        let q = search.read().to_lowercase();
        if q.is_empty() {
            s.contacts.clone()
        } else {
            s.contacts
                .iter()
                .filter(|c| {
                    c.nickname
                        .as_ref()
                        .map(|n| n.to_lowercase().contains(&q))
                        .unwrap_or(false)
                        || c.public_key.to_lowercase().contains(&q)
                })
                .cloned()
                .collect()
        }
    };

    let has_contacts = !state.read().contacts.is_empty();

    rsx! {
        div {
            style: "
                width: 100%;
                height: 100vh;
                display: flex;
                flex-direction: column;
                background: {DARK.bg_primary};
                position: relative;
            ",

            // ─── Top Bar ─────────────────────────
            div {
                style: "
                    height: 64px;
                    display: flex;
                    align-items: center;
                    justify-content: space-between;
                    padding: 0 20px;
                    flex-shrink: 0;
                    background: {DARK.glass_bg};
                    backdrop-filter: blur(24px);
                    -webkit-backdrop-filter: blur(24px);
                    border-bottom: 1px solid {DARK.glass_border};
                    z-index: 10;
                ",

                // Profile button
                button {
                    style: "
                        width: 36px;
                        height: 36px;
                        border-radius: 12px;
                        background: {DARK.bg_tertiary};
                        border: 1px solid {DARK.border_subtle};
                        color: {DARK.text_primary};
                        font-size: 16px;
                        cursor: pointer;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        transition: all 200ms cubic-bezier(.22,1,.36,1);
                    ",
                    onclick: move |_| { nav.push(Route::Profile {}); },
                    "≡"
                }

                // Title with subtle gradient text
                span {
                    style: "
                        font-size: 20px;
                        font-weight: 800;
                        background: {DARK.text_gradient};
                        -webkit-background-clip: text;
                        -webkit-text-fill-color: transparent;
                        background-clip: text;
                        letter-spacing: 0.06em;
                    ",
                    "WRAITH"
                }

                // New chat
                button {
                    style: "
                        width: 36px;
                        height: 36px;
                        border-radius: 12px;
                        background: {DARK.accent_primary};
                        border: none;
                        color: white;
                        font-size: 16px;
                        cursor: pointer;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        transition: all 200ms cubic-bezier(.22,1,.36,1);
                        box-shadow: 0 2px 12px {DARK.accent_primary}44;
                    ",
                    onclick: move |_| { nav.push(Route::NewChat {}); },
                    "✏️"
                }
            }

            // ─── Status Pill ─────────────────────
            StatusBar {}

            // ─── Search ──────────────────────────
            div {
                style: "
                    padding: 4px 16px 14px 16px;
                    flex-shrink: 0;
                ",
                div {
                    style: "
                        position: relative;
                        display: flex;
                        align-items: center;
                    ",

                    // Search icon
                    span {
                        style: "
                            position: absolute;
                            left: 14px;
                            font-size: 14px;
                            color: {DARK.text_tertiary};
                            pointer-events: none;
                            z-index: 1;
                        ",
                        "🔍"
                    }

                    input {
                        class: "input-field",
                        style: "
                            background: {DARK.glass_bg};
                            backdrop-filter: blur(16px);
                            -webkit-backdrop-filter: blur(16px);
                            border: 1px solid {DARK.glass_border};
                            border-radius: 9999px;
                            padding: 12px 16px 12px 40px;
                            font-size: 14px;
                            transition: border-color 200ms ease, box-shadow 200ms ease;
                        ",
                        placeholder: "Search conversations...",
                        value: "{search}",
                        oninput: move |evt| search.set(evt.value()),
                    }
                }
            }

            // ─── Chat List ──────────────────────
            div {
                style: "
                    flex: 1;
                    overflow-y: auto;
                    padding: 0 0 88px 0;
                ",

                if has_contacts {
                    for (i, contact) in contacts.iter().enumerate() {
                        ChatListItem {
                            key: "{contact.id}",
                            contact: contact.clone(),
                            delay_ms: (i as u32) * 50,
                            on_click: {
                                let id = contact.id.clone();
                                move |_| {
                                    nav.push(Route::Chat { contact_id: id.clone() });
                                }
                            },
                        }
                    }
                } else {
                    // ─── Empty State ──────────────
                    div {
                        style: "
                            display: flex;
                            flex-direction: column;
                            align-items: center;
                            justify-content: center;
                            padding: 72px 32px;
                            animation: {presets::fade_in_up()};
                        ",

                        // Animated ghost icon with glow
                        div {
                            style: "
                                font-size: 72px;
                                margin-bottom: 20px;
                                animation: {presets::float()};
                                filter: drop-shadow(0 8px 24px {DARK.accent_primary}33);
                            ",
                            "👻"
                        }

                        h3 {
                            style: "
                                font-size: 20px;
                                font-weight: 700;
                                color: {DARK.text_primary};
                                margin-bottom: 8px;
                            ",
                            "No messages yet"
                        }

                        p {
                            style: "
                                font-size: 14px;
                                color: {DARK.text_secondary};
                                text-align: center;
                                margin-bottom: 28px;
                                line-height: 1.5;
                                max-width: 260px;
                            ",
                            "Add a contact to start a secure, encrypted conversation"
                        }

                        button {
                            class: "btn-primary",
                            style: "
                                width: auto;
                                padding: 14px 36px;
                                font-size: 15px;
                            ",
                            onclick: move |_| { nav.push(Route::NewChat {}); },
                            "＋ New Conversation"
                        }
                    }
                }
            }

            // ─── Bottom Nav ──────────────────────
            BottomNav {}
        }
    }
}

/// Bottom navigation bar with glass effect.
#[component]
fn BottomNav() -> Element {
    let nav = use_navigator();

    rsx! {
        div {
            style: "
                position: fixed;
                bottom: 0;
                left: 0;
                right: 0;
                height: 72px;
                background: {DARK.glass_bg};
                backdrop-filter: blur(24px);
                -webkit-backdrop-filter: blur(24px);
                border-top: 1px solid {DARK.glass_border};
                display: flex;
                align-items: center;
                justify-content: space-around;
                padding: 0 24px;
                padding-bottom: env(safe-area-inset-bottom, 0px);
                z-index: 100;
            ",

            NavButton {
                icon: "💬",
                label: "Chats",
                active: true,
                on_click: move |_| { nav.push(Route::Home {}); },
            }
            NavButton {
                icon: "👤",
                label: "Profile",
                active: false,
                on_click: move |_| { nav.push(Route::Profile {}); },
            }
            NavButton {
                icon: "⚙️",
                label: "Settings",
                active: false,
                on_click: move |_| { nav.push(Route::Settings {}); },
            }
        }
    }
}

#[component]
fn NavButton(
    icon: &'static str,
    label: &'static str,
    active: bool,
    on_click: EventHandler<()>,
) -> Element {
    let color = if active { DARK.accent_primary } else { DARK.text_tertiary };
    let bg = if active {
        format!("{}18", DARK.accent_primary)
    } else {
        "transparent".to_string()
    };
    let font_weight = if active { "700" } else { "500" };

    rsx! {
        button {
            style: "
                background: none;
                border: none;
                display: flex;
                flex-direction: column;
                align-items: center;
                gap: 4px;
                cursor: pointer;
                padding: 8px 20px;
                border-radius: 16px;
                transition: all 200ms cubic-bezier(.22,1,.36,1);
                position: relative;
            ",
            onclick: move |_| on_click.call(()),

            // Active indicator pill
            if active {
                div {
                    style: "
                        position: absolute;
                        top: 0;
                        left: 50%;
                        transform: translateX(-50%);
                        width: 24px;
                        height: 3px;
                        border-radius: 0 0 3px 3px;
                        background: {DARK.accent_primary};
                    ",
                }
            }

            div {
                style: "
                    width: 40px;
                    height: 28px;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    border-radius: 10px;
                    background: {bg};
                    transition: background 200ms ease;
                ",
                span { style: "font-size: 18px;", "{icon}" }
            }

            span {
                style: "
                    font-size: 11px;
                    font-weight: {font_weight};
                    color: {color};
                    transition: color 200ms ease;
                ",
                "{label}"
            }
        }
    }
}
