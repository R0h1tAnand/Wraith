use dioxus::prelude::*;
use crate::ui::theme::DARK;

/// Visual status badge types.
#[derive(Clone, PartialEq)]
pub enum BadgeVariant {
    /// Green gradient — secure / verified / online.
    Success,
    /// Amber gradient — syncing / pending / away.
    Warning,
    /// Red gradient — error / offline / disconnected.
    Error,
    /// Purple accent — info / new / default.
    Info,
}

/// A micro-status pill with gradient fill and subtle glow.
#[component]
pub fn StatusBadge(
    /// Badge label text.
    label: String,
    /// Visual variant (Success, Warning, Error, Info).
    #[props(default = BadgeVariant::Info)]
    variant: BadgeVariant,
    /// Show pulsing dot before label.
    #[props(default = false)]
    pulse: bool,
) -> Element {
    let (bg, glow) = match variant {
        BadgeVariant::Success => (
            DARK.gradient_success,
            "0 0 10px rgba(56, 217, 169, 0.25)",
        ),
        BadgeVariant::Warning => (
            DARK.gradient_warning,
            "0 0 10px rgba(247, 181, 56, 0.25)",
        ),
        BadgeVariant::Error => (
            DARK.gradient_error,
            "0 0 10px rgba(247, 85, 85, 0.25)",
        ),
        BadgeVariant::Info => (
            DARK.gradient_accent,
            DARK.shadow_glow_accent,
        ),
    };

    rsx! {
        span {
            style: "
                display: inline-flex;
                align-items: center;
                gap: 5px;
                padding: 4px 10px;
                border-radius: 20px;
                background: {bg};
                color: {on_accent};
                font-size: 11px;
                font-weight: 700;
                letter-spacing: 0.03em;
                text-transform: uppercase;
                box-shadow: {glow};
                white-space: nowrap;
            ",
            on_accent = DARK.text_on_accent,

            if pulse {
                span {
                    style: "
                        width: 6px;
                        height: 6px;
                        border-radius: 50%;
                        background: {on_accent};
                        animation: breathe 2s ease-in-out infinite;
                    ",
                    on_accent = DARK.text_on_accent,
                }
            }

            "{label}"
        }
    }
}
