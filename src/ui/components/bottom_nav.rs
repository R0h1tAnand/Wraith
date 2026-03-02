use dioxus::prelude::*;
use crate::app::Route;
use crate::ui::theme::DARK;

/// Bottom tab navigation bar.
#[component]
pub fn BottomNav(active_tab: Option<String>) -> Element {
    let nav = use_navigator();
    let current = active_tab.unwrap_or_else(|| "chats".to_string());

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

            NavTab {
                icon: "💬",
                label: "Chats",
                active: current == "chats",
                on_click: move |_| { nav.push(Route::Home {}); },
            }
            NavTab {
                icon: "👤",
                label: "Profile",
                active: current == "profile",
                on_click: move |_| { nav.push(Route::Profile {}); },
            }
            NavTab {
                icon: "⚙️",
                label: "Settings",
                active: current == "settings",
                on_click: move |_| { nav.push(Route::Settings {}); },
            }
        }
    }
}

#[component]
fn NavTab(
    icon: &'static str,
    label: &'static str,
    active: bool,
    on_click: EventHandler<()>,
) -> Element {
    let color = if active { DARK.accent_primary } else { DARK.text_secondary };
    let weight = if active { "600" } else { "500" };

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
                style: "
                    font-size: 11px;
                    font-weight: {weight};
                    color: {color};
                ",
                "{label}"
            }
        }
    }
}
