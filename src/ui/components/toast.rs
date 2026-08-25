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
    /// Gradient background for the kind.
    pub fn gradient(&self) -> &'static str {
        match self {
            ToastKind::Success => DARK.gradient_success,
            ToastKind::Error   => DARK.gradient_error,
            ToastKind::Info    => DARK.gradient_accent,
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            ToastKind::Success => "✓",
            ToastKind::Error   => "✕",
            ToastKind::Info    => "ℹ",
        }
    }
}

/// Glassmorphic toast notification with gradient accent stripe.
///
/// Slides in from top with a soft entrance.
#[component]
pub fn Toast(
    message: String,
    kind: ToastKind,
    visible: bool,
) -> Element {
    if !visible {
        return rsx! {};
    }

    let gradient = kind.gradient();
    let icon = kind.icon();

    rsx! {
        div {
            style: "
                position: fixed;
                top: max(16px, env(safe-area-inset-top, 16px));
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
                    background: {glass_bg};
                    border: 1px solid {glass_border};
                    border-radius: 16px;
                    backdrop-filter: {backdrop};
                    -webkit-backdrop-filter: {backdrop};
                    box-shadow: {shadow};
                    max-width: 400px;
                    overflow: hidden;
                    position: relative;
                ",
                glass_bg = DARK.glass_bg,
                glass_border = DARK.glass_border,
                backdrop = DARK.glass_backdrop,
                shadow = DARK.shadow_elevated,

                // Left accent stripe
                div {
                    style: "
                        position: absolute;
                        left: 0;
                        top: 0;
                        bottom: 0;
                        width: 3px;
                        background: {gradient};
                    ",
                }

                // Icon badge
                div {
                    style: "
                        width: 26px;
                        height: 26px;
                        border-radius: 50%;
                        background: {gradient};
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        font-size: 12px;
                        font-weight: 700;
                        color: {on_accent};
                        flex-shrink: 0;
                    ",
                    on_accent = DARK.text_on_accent,
                    "{icon}"
                }

                span {
                    style: "
                        font-size: 14px;
                        font-weight: 500;
                        color: {text_pri};
                    ",
                    text_pri = DARK.text_primary,
                    "{message}"
                }
            }
        }
    }
}
