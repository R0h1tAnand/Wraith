use dioxus::prelude::*;
use crate::ui::theme::DARK;
use crate::ui::animations::presets;

/// Glassmorphic slide-up modal sheet with backdrop blur.
#[component]
pub fn Modal(
    show: bool,
    on_close: EventHandler<()>,
    title: Option<String>,
    children: Element,
) -> Element {
    if !show {
        return rsx! {};
    }

    rsx! {
        // Backdrop
        div {
            style: "
                position: fixed;
                top: 0;
                left: 0;
                right: 0;
                bottom: 0;
                background: rgba(0, 0, 0, 0.55);
                backdrop-filter: blur(6px);
                -webkit-backdrop-filter: blur(6px);
                z-index: 200;
                display: flex;
                flex-direction: column;
                justify-content: flex-end;
                animation: fadeIn 200ms ease both;
            ",
            onclick: move |_| on_close.call(()),

            // Sheet
            div {
                style: "
                    background: {glass_bg};
                    border: 1px solid {glass_border};
                    border-bottom: none;
                    border-radius: 24px 24px 0 0;
                    backdrop-filter: {backdrop};
                    -webkit-backdrop-filter: {backdrop};
                    padding: 8px 0 0 0;
                    max-height: 85vh;
                    animation: {slide_up};
                    box-shadow: 0 -8px 40px rgba(0, 0, 0, 0.3);
                ",
                glass_bg = DARK.glass_bg,
                glass_border = DARK.glass_border,
                backdrop = DARK.glass_backdrop,
                slide_up = presets::slide_up(),
                onclick: move |evt| evt.stop_propagation(),

                // Handle bar
                div {
                    style: "
                        display: flex;
                        justify-content: center;
                        padding: 8px 0 16px 0;
                    ",
                    div {
                        style: "
                            width: 36px;
                            height: 4px;
                            border-radius: 2px;
                            background: {DARK.surface_elevated};
                        ",
                    }
                }

                // Title
                if let Some(title) = &title {
                    div {
                        style: "
                            padding: 0 20px 16px 20px;
                            font-size: 18px;
                            font-weight: 700;
                            letter-spacing: -0.02em;
                            background: {gradient};
                            -webkit-background-clip: text;
                            -webkit-text-fill-color: transparent;
                            background-clip: text;
                        ",
                        gradient = DARK.gradient_text,
                        "{title}"
                    }
                }

                // Content
                div {
                    style: "
                        padding: 0 20px 24px 20px;
                        padding-bottom: max(24px, env(safe-area-inset-bottom, 24px));
                        overflow-y: auto;
                    ",
                    {children}
                }
            }
        }
    }
}
