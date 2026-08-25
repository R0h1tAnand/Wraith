use dioxus::prelude::*;
use crate::ui::theme::DARK;

/// Top navigation bar with glass surface, gradient title, and back navigation.
#[component]
pub fn HeaderBar(
    /// Title text.
    title: String,
    /// Optional subtitle (e.g. "online", "typing…").
    #[props(default)]
    subtitle: Option<String>,
    /// Show back arrow.
    #[props(default = false)]
    show_back: bool,
    /// Back button handler.
    #[props(default)]
    on_back: EventHandler<()>,
    /// Right side action slot.
    children: Element,
) -> Element {
    rsx! {
        header {
            style: "
                display: flex;
                align-items: center;
                padding: 12px 16px;
                padding-top: max(12px, env(safe-area-inset-top, 12px));
                gap: 12px;
                background: {glass_bg};
                border-bottom: 1px solid {glass_border};
                backdrop-filter: {backdrop};
                -webkit-backdrop-filter: {backdrop};
                position: sticky;
                top: 0;
                z-index: 100;
            ",
            glass_bg = DARK.glass_bg,
            glass_border = DARK.glass_border,
            backdrop = DARK.glass_backdrop,

            // Back button
            if show_back {
                button {
                    style: "
                        width: 34px;
                        height: 34px;
                        border-radius: 10px;
                        border: 1px solid {border};
                        background: transparent;
                        color: {accent};
                        font-size: 18px;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        cursor: pointer;
                        transition: background 150ms ease;
                        flex-shrink: 0;
                    ",
                    border = DARK.glass_border,
                    accent = DARK.accent_primary,
                    onclick: move |_| on_back.call(()),
                    "←"
                }
            }

            // Title area
            div {
                style: "
                    flex: 1;
                    min-width: 0;
                    display: flex;
                    flex-direction: column;
                    gap: 1px;
                ",

                span {
                    style: "
                        font-size: 17px;
                        font-weight: 700;
                        letter-spacing: -0.03em;
                        background: {gradient};
                        -webkit-background-clip: text;
                        -webkit-text-fill-color: transparent;
                        background-clip: text;
                        overflow: hidden;
                        text-overflow: ellipsis;
                        white-space: nowrap;
                    ",
                    gradient = DARK.gradient_text,
                    "{title}"
                }

                if let Some(ref sub) = subtitle {
                    span {
                        style: "
                            font-size: 12px;
                            color: {text_sec};
                            letter-spacing: 0.01em;
                        ",
                        text_sec = DARK.text_secondary,
                        "{sub}"
                    }
                }
            }

            // Right side actions
            div {
                style: "
                    display: flex;
                    align-items: center;
                    gap: 8px;
                    flex-shrink: 0;
                ",
                {children}
            }
        }
    }
}
