use dioxus::prelude::*;
use crate::ui::theme::DARK;

/// Toast notification type.
#[derive(Debug, Clone, PartialEq)]
pub enum ToastKind {
    Success,
    Error,
    Info,
}

impl ToastKind {
    pub fn color(&self) -> &'static str {
        match self {
            ToastKind::Success => "#4ADE80",
            ToastKind::Error => "#F87171",
            ToastKind::Info => "#7C6AF7",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            ToastKind::Success => "✓",
            ToastKind::Error => "✕",
            ToastKind::Info => "ℹ",
        }
    }
}

/// Toast notification component.
///
/// Slides in from top and auto-dismisses.
#[component]
pub fn Toast(
    message: String,
    kind: ToastKind,
    visible: bool,
) -> Element {
    if !visible {
        return rsx! {};
    }

    let color = kind.color();
    let icon = kind.icon();
    let bg_alpha = "1A"; // ~10% opacity
    let bg = format!("{}{}", color, bg_alpha);

    rsx! {
        div {
            style: "
                position: fixed;
                top: 16px;
                left: 16px;
                right: 16px;
                z-index: 300;
                display: flex;
                justify-content: center;
                animation: fadeInUp 300ms cubic-bezier(0.16, 1, 0.3, 1) both;
            ",

            div {
                style: "
                    display: inline-flex;
                    align-items: center;
                    gap: 10px;
                    padding: 12px 20px;
                    background: {DARK.bg_secondary};
                    border: 1px solid {color}33;
                    border-radius: 16px;
                    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
                ",

                // Icon
                div {
                    style: "
                        width: 24px;
                        height: 24px;
                        border-radius: 50%;
                        background: {bg};
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        font-size: 12px;
                        font-weight: 700;
                        color: {color};
                        flex-shrink: 0;
                    ",
                    "{icon}"
                }

                span {
                    style: "
                        font-size: 14px;
                        font-weight: 500;
                        color: {DARK.text_primary};
                    ",
                    "{message}"
                }
            }
        }
    }
}
