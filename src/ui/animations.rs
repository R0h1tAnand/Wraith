/// Animation helpers — CSS animation strings for inline styling.

/// Easing curves.
pub mod easing {
    pub const EASE_OUT_EXPO: &str = "cubic-bezier(0.16, 1, 0.3, 1)";
    pub const EASE_OUT_BACK: &str = "cubic-bezier(0.34, 1.56, 0.64, 1)";
    pub const EASE_OUT: &str = "ease-out";
    pub const EASE_IN_OUT: &str = "ease-in-out";
    pub const LINEAR: &str = "linear";
}

/// Pre-built animation strings for the `animation` CSS property.
pub mod presets {
    use super::easing;

    pub fn fade_in_up() -> String {
        format!("fadeInUp 300ms {} both", easing::EASE_OUT_EXPO)
    }

    pub fn fade_in_up_delayed(delay_ms: u32) -> String {
        format!(
            "fadeInUp 300ms {} {}ms both",
            easing::EASE_OUT_EXPO, delay_ms
        )
    }

    pub fn slide_up() -> String {
        format!("slideUp 350ms {} both", easing::EASE_OUT_EXPO)
    }

    pub fn slide_left() -> String {
        format!("slideLeft 300ms {} both", easing::EASE_OUT_EXPO)
    }

    pub fn message_pop() -> String {
        format!("messagePop 280ms {} both", easing::EASE_OUT_BACK)
    }

    pub fn pulse_glow() -> String {
        format!("pulseGlow 2s {} infinite", easing::EASE_IN_OUT)
    }

    pub fn aura_pulse() -> String {
        format!("auraPulse 3s {} infinite", easing::EASE_IN_OUT)
    }

    pub fn spin() -> String {
        format!("spin 1s {} infinite", easing::LINEAR)
    }

    pub fn shimmer() -> String {
        format!("shimmer 1.5s {} infinite", easing::LINEAR)
    }

    pub fn scale_in() -> String {
        format!("scaleIn 400ms {} both", easing::EASE_OUT_EXPO)
    }

    pub fn fade_out() -> String {
        "fadeOut 300ms ease both".to_string()
    }
}

/// Transition helpers for inline `transition` properties.
pub mod transitions {
    pub const ALL_FAST: &str = "all 150ms ease";
    pub const ALL_NORMAL: &str = "all 200ms ease";
    pub const BACKGROUND_FAST: &str = "background 150ms ease";
    pub const TRANSFORM_FAST: &str = "transform 150ms ease";
    pub const OPACITY_FAST: &str = "opacity 200ms ease";
}
