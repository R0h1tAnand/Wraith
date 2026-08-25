use dioxus::prelude::*;
use crate::ui::theme::DARK;

/// Message composer bar with glass surface, animated send button,
/// and input focus glow.
#[component]
pub fn InputBar(
    /// Current input value (controlled).
    value: String,
    /// Called when the input value changes.
    on_input: EventHandler<String>,
    /// Called when the user taps send.
    on_send: EventHandler<()>,
    /// Called when the attachment button is tapped.
    #[props(default)]
    on_attach: EventHandler<()>,
) -> Element {
    let has_text = !value.trim().is_empty();

    let send_bg = if has_text {
        DARK.gradient_accent
    } else {
        "transparent"
    };

    let send_opacity = if has_text { "1" } else { "0.3" };

    rsx! {
        div {
            style: "
                display: flex;
                align-items: flex-end;
                gap: 8px;
                padding: 10px 12px;
                padding-bottom: max(10px, env(safe-area-inset-bottom, 10px));
                background: {glass_bg};
                border-top: 1px solid {glass_border};
                backdrop-filter: {backdrop};
                -webkit-backdrop-filter: {backdrop};
            ",
            glass_bg = DARK.glass_bg,
            glass_border = DARK.glass_border,
            backdrop = DARK.glass_backdrop,

            // Attachment button
            button {
                style: "
                    width: 38px;
                    height: 38px;
                    border-radius: 12px;
                    border: 1px solid {border};
                    background: {btn_bg};
                    color: {text_sec};
                    font-size: 18px;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    cursor: pointer;
                    transition: background 150ms ease, border-color 150ms ease;
                    flex-shrink: 0;
                    backdrop-filter: {backdrop};
                    -webkit-backdrop-filter: {backdrop};
                ",
                border = DARK.glass_border,
                btn_bg = DARK.glass_bg,
                text_sec = DARK.text_secondary,
                backdrop = DARK.glass_backdrop,
                onclick: move |_| on_attach.call(()),
                "+"
            }

            // Text input
            input {
                class: "input-field",
                style: "
                    flex: 1;
                    min-height: 38px;
                    padding: 8px 14px;
                    border-radius: 14px;
                    font-size: 15px;
                    line-height: 1.4;
                    letter-spacing: -0.01em;
                    transition: border-color 200ms ease, box-shadow 200ms ease;
                ",
                r#type: "text",
                placeholder: "Message…",
                value: "{value}",
                oninput: move |e| on_input.call(e.value()),
            }

            // Send button
            button {
                style: "
                    width: 38px;
                    height: 38px;
                    border-radius: 50%;
                    border: none;
                    background: {send_bg};
                    color: {on_accent};
                    font-size: 16px;
                    font-weight: 700;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    cursor: pointer;
                    opacity: {send_opacity};
                    transition: opacity 200ms ease, transform 150ms ease, box-shadow 200ms ease;
                    flex-shrink: 0;
                    box-shadow: {glow};
                ",
                on_accent = DARK.text_on_accent,
                glow = if has_text { DARK.shadow_glow_accent } else { "none" },
                disabled: !has_text,
                onclick: move |_| {
                    if has_text {
                        on_send.call(());
                    }
                },
                "↑"
            }
        }
    }
}
