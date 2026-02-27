use dioxus::prelude::*;
use crate::ui::theme::DARK;
use crate::ui::animations::presets;

/// Slide-up modal sheet.
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
                background: rgba(0, 0, 0, 0.6);
                z-index: 200;
                display: flex;
                flex-direction: column;
                justify-content: flex-end;
            ",
            onclick: move |_| on_close.call(()),

            // Sheet
            div {
                style: "
                    background: {DARK.bg_secondary};
                    border-top: 1px solid {DARK.border_subtle};
                    border-radius: 24px 24px 0 0;
                    padding: 8px 0 0 0;
                    max-height: 80vh;
                    animation: {presets::slide_up()};
                ",
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
                            background: {DARK.bg_tertiary};
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
                            color: {DARK.text_primary};
                        ",
                        "{title}"
                    }
                }

                // Content
                div {
                    style: "
                        padding: 0 20px 24px 20px;
                        overflow-y: auto;
                    ",
                    {children}
                }
            }
        }
    }
}
