use dioxus::prelude::*;
use crate::app::Route;
use crate::ui::theme::DARK;

/// Glassmorphic bottom tab bar with animated active indicator.
#[component]
pub fn BottomNav(active_tab: Option<String>) -> Element {
    let nav = use_navigator();
    let current = active_tab.unwrap_or_else(|| "chats".to_string());

    rsx! {
        nav {
            style: "
                position: fixed;
                bottom: 0;
                left: 0;
                right: 0;
                height: 68px;
                background: {glass_bg};
                border-top: 1px solid {glass_border};
                backdrop-filter: {backdrop};
                -webkit-backdrop-filter: {backdrop};
                display: flex;
                align-items: center;
                justify-content: space-around;
                padding: 0 24px;
                padding-bottom: env(safe-area-inset-bottom, 0px);
                z-index: 100;
            ",
            glass_bg = DARK.glass_bg,
            glass_border = DARK.glass_border,
            backdrop = DARK.glass_backdrop,

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
    let (color, weight, bg, shadow) = if active {
        (
            DARK.text_on_accent,
            "700",
            DARK.gradient_accent,
            DARK.shadow_glow_accent,
        )
    } else {
        (
            DARK.text_secondary,
            "500",
            "transparent",
            "none",
        )
    };

    rsx! {
        button {
            style: "
                background: none;
                border: none;
                display: flex;
                flex-direction: column;
                align-items: center;
                gap: 3px;
                cursor: pointer;
                padding: 6px 18px;
                position: relative;
                transition: all 200ms ease;
            ",
            onclick: move |_| on_click.call(()),

            // Active pill background behind icon
            if active {
                div {
                    style: "
                        position: absolute;
                        top: 0;
                        left: 50%;
                        transform: translateX(-50%);
                        width: 44px;
                        height: 28px;
                        border-radius: 14px;
                        background: {bg};
                        box-shadow: {shadow};
                        z-index: 0;
                    ",
                }
            }

            span {
                style: "
                    font-size: 18px;
                    position: relative;
                    z-index: 1;
                    filter: {filter};
                ",
                filter = if active { "brightness(1.3)" } else { "none" },
                "{icon}"
            }
            span {
                style: "
                    font-size: 10px;
                    font-weight: {weight};
                    color: {color};
                    letter-spacing: 0.02em;
                    position: relative;
                    z-index: 1;
                ",
                "{label}"
            }
        }
    }
}
