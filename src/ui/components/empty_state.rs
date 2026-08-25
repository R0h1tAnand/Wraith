use dioxus::prelude::*;
use crate::ui::theme::DARK;
use crate::ui::animations::presets;

/// Empty state placeholder with icon, title, subtitle, and optional CTA.
///
/// Uses the `glass_appear` animation for an elegant entrance.
#[component]
pub fn EmptyState(
    /// Large emoji / icon character.
    #[props(default = "💬")]
    icon: &'static str,
    /// Headline.
    title: String,
    /// Supporting description.
    #[props(default = String::new())]
    subtitle: String,
    /// Optional call-to-action button label.
    #[props(default)]
    action_label: Option<String>,
    /// CTA handler.
    #[props(default)]
    on_action: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            style: "
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                padding: 60px 32px;
                text-align: center;
                gap: 16px;
                animation: {anim};
            ",
            anim = presets::GLASS_APPEAR,

            // Icon
            div {
                style: "
                    font-size: 56px;
                    margin-bottom: 4px;
                    filter: drop-shadow(0 4px 12px rgba(124, 106, 247, 0.15));
                ",
                "{icon}"
            }

            // Title
            span {
                style: "
                    font-size: 20px;
                    font-weight: 700;
                    letter-spacing: -0.03em;
                    background: {gradient};
                    -webkit-background-clip: text;
                    -webkit-text-fill-color: transparent;
                    background-clip: text;
                ",
                gradient = DARK.gradient_text,
                "{title}"
            }

            // Subtitle
            if !subtitle.is_empty() {
                p {
                    style: "
                        font-size: 14px;
                        color: {text_sec};
                        line-height: 1.5;
                        max-width: 280px;
                    ",
                    text_sec = DARK.text_secondary,
                    "{subtitle}"
                }
            }

            // CTA Button
            if let Some(ref label) = action_label {
                button {
                    style: "
                        margin-top: 8px;
                        padding: 10px 24px;
                        border-radius: 14px;
                        border: none;
                        background: {gradient_btn};
                        color: {on_accent};
                        font-size: 14px;
                        font-weight: 600;
                        cursor: pointer;
                        box-shadow: {glow};
                        transition: transform 100ms ease, box-shadow 200ms ease;
                    ",
                    gradient_btn = DARK.gradient_accent,
                    on_accent = DARK.text_on_accent,
                    glow = DARK.shadow_glow_accent,
                    onclick: move |_| on_action.call(()),
                    "{label}"
                }
            }
        }
    }
}
