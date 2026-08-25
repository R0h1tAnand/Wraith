/// Animation presets for Wraith's premium glassmorphic UI.
///
/// Returns CSS `animation` shorthand strings that can be inlined.
/// All keyframes are defined in `global_styles.rs` and injected
/// at the app root.

/// Fade in from below with staggered delay.
pub fn fade_in_up(delay_ms: u32) -> String {
    format!(
        "wraith-fade-in-up 400ms cubic-bezier(0.16, 1, 0.3, 1) {}ms both",
        delay_ms
    )
}

/// Scale + fade for glass surfaces appearing.
pub fn glass_appear(delay_ms: u32) -> String {
    format!(
        "wraith-glass-appear 500ms cubic-bezier(0.34, 1.56, 0.64, 1) {}ms both",
        delay_ms
    )
}

/// Slide up from bottom (modals, sheets).
pub fn slide_up(delay_ms: u32) -> String {
    format!(
        "wraith-slide-up 450ms cubic-bezier(0.16, 1, 0.3, 1) {}ms both",
        delay_ms
    )
}

/// Slide down to exit.
pub fn slide_down() -> String {
    "wraith-slide-down 300ms cubic-bezier(0.7, 0, 0.84, 0) forwards".to_string()
}

/// Fade overlay backdrop in.
pub fn fade_in(delay_ms: u32) -> String {
    format!(
        "wraith-fade-in 300ms ease {}ms both",
        delay_ms
    )
}

/// Slow breathing glow (active indicators, status dots).
pub fn breathe() -> String {
    "wraith-breathe 2s ease-in-out infinite".to_string()
}

/// Scale spring for tap feedback on buttons.
pub fn scale_tap() -> String {
    "wraith-scale-tap 200ms cubic-bezier(0.34, 1.56, 0.64, 1) both".to_string()
}

/// Shimmer loading effect.
pub fn shimmer() -> String {
    "wraith-shimmer 1.5s ease-in-out infinite".to_string()
}

/// Toast enter from top.
pub fn toast_enter() -> String {
    "wraith-toast-enter 400ms cubic-bezier(0.34, 1.56, 0.64, 1) both".to_string()
}

/// Toast exit to top.
pub fn toast_exit() -> String {
    "wraith-toast-exit 250ms cubic-bezier(0.7, 0, 0.84, 0) forwards".to_string()
}

/// Spin animation for loaders.
pub fn spin() -> String {
    "wraith-spin 1s linear infinite".to_string()
}

/// Convenience: fade_in_up with a per-index stagger.
pub fn stagger(index: u32, base_delay_ms: u32) -> String {
    fade_in_up(base_delay_ms + index * 50)
}
