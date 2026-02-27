use dioxus::prelude::*;
use crate::app::Route;
use crate::state::app_state::AppState;
use crate::state::actions;
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
            ",

            // ─── Top Bar ─────────────────────────
            div {
                style: "
                    height: 60px;
                    display: flex;
                    align-items: center;
                    justify-content: space-between;
                    padding: 0 20px;
                    flex-shrink: 0;
                ",

                // Menu icon
                button {
                    style: "
                        background: none;
                        border: none;
                        color: {DARK.text_primary};
                        font-size: 20px;
                        cursor: pointer;
                        padding: 8px;
                        border-radius: 12px;
                        transition: background 150ms ease;
                    ",
                    onclick: move |_| nav.push(Route::Profile {}),
                    "≡"
                }

                // Title
                span {
                    style: "
                        font-size: 18px;
                        font-weight: 700;
                        color: {DARK.text_primary};
                        letter-spacing: 0.05em;
                    ",
                    "Wraith"
                }

                // New chat button
                button {
                    style: "
                        background: none;
                        border: none;
                        color: {DARK.accent_primary};
                        font-size: 20px;
                        cursor: pointer;
                        padding: 8px;
                        border-radius: 12px;
                        transition: background 150ms ease;
                    ",
                    onclick: move |_| nav.push(Route::NewChat {}),
                    "✏️"
                }
            }

            // ─── Status Pill ─────────────────────
            StatusBar {}

            // ─── Search ──────────────────────────
            div {
                style: "
                    padding: 0 16px 12px 16px;
                    flex-shrink: 0;
                ",
                input {
                    class: "input-field",
                    style: "
                        background: {DARK.bg_secondary};
                        border-radius: 9999px;
                        padding: 12px 16px;
                        font-size: 14px;
                    ",
                    placeholder: "Search conversations...",
                    value: "{search}",
                    oninput: move |evt| search.set(evt.value()),
                }
            }

            // ─── Chat List ──────────────────────
            div {
                style: "
                    flex: 1;
                    overflow-y: auto;
                    padding: 0 0 80px 0;
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
                    // Empty state
                    div {
                        style: "
                            display: flex;
                            flex-direction: column;
                            align-items: center;
                            justify-content: center;
                            padding: 64px 32px;
                            animation: {presets::fade_in_up()};
                        ",

                        div {
                            style: "
                                font-size: 64px;
                                margin-bottom: 16px;
                                opacity: 0.6;
                            ",
                            "👻"
                        }

                        h3 {
                            style: "
                                font-size: 18px;
                                font-weight: 600;
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
                                margin-bottom: 24px;
                            ",
                            "Add a contact to start a secure conversation"
                        }

                        button {
                            class: "btn-primary",
                            style: "width: auto; padding: 12px 32px;",
                            onclick: move |_| nav.push(Route::NewChat {}),
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

/// Bottom navigation bar.
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
                height: 68px;
                background: {DARK.bg_secondary};
                border-top: 1px solid {DARK.border_subtle};
                display: flex;
                align-items: center;
                justify-content: space-around;
                padding: 0 16px;
                padding-bottom: env(safe-area-inset-bottom, 0px);
                z-index: 100;
            ",

            NavButton {
                icon: "💬",
                label: "Chats",
                active: true,
                on_click: move |_| nav.push(Route::Home {}),
            }
            NavButton {
                icon: "👤",
                label: "Profile",
                active: false,
                on_click: move |_| nav.push(Route::Profile {}),
            }
            NavButton {
                icon: "⚙️",
                label: "Settings",
                active: false,
                on_click: move |_| nav.push(Route::Settings {}),
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
    let color = if active { DARK.accent_primary } else { DARK.text_secondary };

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
                padding: 8px 16px;
                transition: all 150ms ease;
            ",
            onclick: move |_| on_click.call(()),

            span { style: "font-size: 20px;", "{icon}" }
            span {
                style: "font-size: 11px; font-weight: 500; color: {color};",
                "{label}"
            }
        }
    }
}
