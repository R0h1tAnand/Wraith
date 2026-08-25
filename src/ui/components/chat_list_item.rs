use dioxus::prelude::*;
use crate::ui::theme::DARK;
use crate::ui::animations::presets;
use crate::ui::components::avatar::Avatar;

/// A single conversation row in the chat list.
///
/// Glass card surface with gradient unread badge and smooth stagger animation.
#[component]
pub fn ChatListItem(
    /// Display name of the contact.
    name: String,
    /// Last message preview text.
    last_message: String,
    /// Timestamp or relative time string (e.g. "2m ago").
    time: String,
    /// Number of unread messages (0 = no badge).
    #[props(default = 0)]
    unread: u32,
    /// Public key for avatar generation.
    #[props(default = String::new())]
    pubkey: String,
    /// Position in the list for stagger animation.
    #[props(default = 0)]
    index: u32,
    /// Tap handler.
    #[props(default)]
    on_click: EventHandler<()>,
) -> Element {
    let anim = presets::stagger(index, 40);
    let key = if pubkey.is_empty() { name.clone() } else { pubkey };

    rsx! {
        div {
            style: "
                display: flex;
                align-items: center;
                gap: 14px;
                padding: 14px 16px;
                margin: 0 12px 4px;
                border-radius: 16px;
                background: {glass_bg};
                border: 1px solid {glass_border};
                backdrop-filter: {backdrop};
                -webkit-backdrop-filter: {backdrop};
                cursor: pointer;
                transition: background 200ms ease, border-color 200ms ease, box-shadow 200ms ease;
                animation: {anim};
                -webkit-tap-highlight-color: transparent;
            ",
            glass_bg = DARK.glass_bg,
            glass_border = DARK.glass_border,
            backdrop = DARK.glass_backdrop,
            onclick: move |_| on_click.call(()),

            // Avatar
            Avatar { pubkey: key, size: 46 }

            // Content area
            div {
                style: "
                    flex: 1;
                    min-width: 0;
                    display: flex;
                    flex-direction: column;
                    gap: 4px;
                ",

                // Top row: name + time
                div {
                    style: "
                        display: flex;
                        justify-content: space-between;
                        align-items: baseline;
                    ",

                    span {
                        style: "
                            font-size: 15px;
                            font-weight: 600;
                            color: {text};
                            letter-spacing: -0.02em;
                            overflow: hidden;
                            text-overflow: ellipsis;
                            white-space: nowrap;
                        ",
                        text = DARK.text_primary,
                        "{name}"
                    }

                    span {
                        style: "
                            font-size: 12px;
                            color: {text_tert};
                            flex-shrink: 0;
                            margin-left: 8px;
                        ",
                        text_tert = DARK.text_tertiary,
                        "{time}"
                    }
                }

                // Bottom row: preview + badge
                div {
                    style: "
                        display: flex;
                        justify-content: space-between;
                        align-items: center;
                    ",

                    span {
                        style: "
                            font-size: 13px;
                            color: {text_sec};
                            overflow: hidden;
                            text-overflow: ellipsis;
                            white-space: nowrap;
                            flex: 1;
                            line-height: 1.4;
                        ",
                        text_sec = DARK.text_secondary,
                        "{last_message}"
                    }

                    if unread > 0 {
                        div {
                            style: "
                                min-width: 20px;
                                height: 20px;
                                padding: 0 6px;
                                border-radius: 10px;
                                background: {gradient};
                                display: flex;
                                align-items: center;
                                justify-content: center;
                                font-size: 11px;
                                font-weight: 700;
                                color: {on_accent};
                                margin-left: 8px;
                                box-shadow: {glow};
                                flex-shrink: 0;
                            ",
                            gradient = DARK.gradient_accent,
                            on_accent = DARK.text_on_accent,
                            glow = DARK.shadow_glow_accent,
                            "{unread}"
                        }
                    }
                }
            }
        }
    }
}
