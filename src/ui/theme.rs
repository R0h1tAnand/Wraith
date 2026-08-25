/// Wraith Design System — Premium Dark Glassmorphic Theme
///
/// Every token is tuned for a cipher-punk aesthetic: deep blacks,
/// frosted glass surfaces, and electric neon accents.

pub struct DarkTheme {
    // ── Background layers (darkest → lightest) ──────────────────
    pub bg_primary: &'static str,
    pub bg_secondary: &'static str,
    pub bg_tertiary: &'static str,
    pub bg_elevated: &'static str,

    // ── Gradient backgrounds ────────────────────────────────────
    pub bg_gradient: &'static str,
    pub bg_gradient_subtle: &'static str,

    // ── Glass surfaces ──────────────────────────────────────────
    pub glass_bg: &'static str,
    pub glass_bg_heavy: &'static str,
    pub glass_border: &'static str,
    pub glass_border_light: &'static str,
    pub glass_shadow: &'static str,
    pub glass_backdrop: &'static str,
    pub glass_backdrop_heavy: &'static str,

    // ── Text hierarchy ──────────────────────────────────────────
    pub text_primary: &'static str,
    pub text_secondary: &'static str,
    pub text_tertiary: &'static str,
    pub text_on_accent: &'static str,

    // ── Accents ─────────────────────────────────────────────────
    pub accent_primary: &'static str,      // Electric violet
    pub accent_primary_dim: &'static str,
    pub accent_cyan: &'static str,         // Neon cyan
    pub accent_cyan_dim: &'static str,
    pub accent_green: &'static str,        // Success / connected
    pub accent_green_dim: &'static str,
    pub accent_amber: &'static str,        // Warning
    pub accent_red: &'static str,          // Danger / error
    pub accent_red_dim: &'static str,

    // ── Gradients (for fills) ───────────────────────────────────
    pub gradient_accent: &'static str,
    pub gradient_accent_hover: &'static str,
    pub gradient_danger: &'static str,

    // ── Borders ─────────────────────────────────────────────────
    pub border_subtle: &'static str,
    pub border_medium: &'static str,

    // ── Shadows ─────────────────────────────────────────────────
    pub shadow_soft: &'static str,
    pub shadow_medium: &'static str,
    pub shadow_glow_accent: &'static str,
    pub shadow_glow_cyan: &'static str,
    pub shadow_glow_green: &'static str,
    pub shadow_glow_red: &'static str,
}

pub static DARK: DarkTheme = DarkTheme {
    // Backgrounds — pure dark to charcoal
    bg_primary:   "#09090B",
    bg_secondary: "#111114",
    bg_tertiary:  "#1A1A1F",
    bg_elevated:  "#222228",

    // Gradient backgrounds
    bg_gradient:        "linear-gradient(145deg, #09090B 0%, #12101E 50%, #09090B 100%)",
    bg_gradient_subtle: "linear-gradient(180deg, #111114 0%, #09090B 100%)",

    // Glass surfaces
    glass_bg:            "rgba(255, 255, 255, 0.04)",
    glass_bg_heavy:      "rgba(255, 255, 255, 0.08)",
    glass_border:        "rgba(255, 255, 255, 0.08)",
    glass_border_light:  "rgba(255, 255, 255, 0.05)",
    glass_shadow:        "0 4px 24px rgba(0, 0, 0, 0.4)",
    glass_backdrop:      "blur(20px) saturate(180%)",
    glass_backdrop_heavy: "blur(40px) saturate(200%)",

    // Text
    text_primary:   "#F4F4F5",
    text_secondary: "#A1A1AA",
    text_tertiary:  "#63636E",
    text_on_accent: "#FFFFFF",

    // Accents
    accent_primary:     "#7C6AF7",
    accent_primary_dim: "rgba(124, 106, 247, 0.15)",
    accent_cyan:        "#06B6D4",
    accent_cyan_dim:    "rgba(6, 182, 212, 0.15)",
    accent_green:       "#22C55E",
    accent_green_dim:   "rgba(34, 197, 94, 0.15)",
    accent_amber:       "#F59E0B",
    accent_red:         "#EF4444",
    accent_red_dim:     "rgba(239, 68, 68, 0.15)",

    // Gradients
    gradient_accent:       "linear-gradient(135deg, #7C6AF7 0%, #06B6D4 100%)",
    gradient_accent_hover: "linear-gradient(135deg, #8B7CF8 0%, #0EC5E3 100%)",
    gradient_danger:       "linear-gradient(135deg, #EF4444 0%, #DC2626 100%)",

    // Borders
    border_subtle: "rgba(255, 255, 255, 0.06)",
    border_medium: "rgba(255, 255, 255, 0.12)",

    // Shadows
    shadow_soft:       "0 2px 12px rgba(0, 0, 0, 0.3)",
    shadow_medium:     "0 8px 32px rgba(0, 0, 0, 0.5)",
    shadow_glow_accent: "0 0 20px rgba(124, 106, 247, 0.3)",
    shadow_glow_cyan:   "0 0 20px rgba(6, 182, 212, 0.3)",
    shadow_glow_green:  "0 0 20px rgba(34, 197, 94, 0.3)",
    shadow_glow_red:    "0 0 16px rgba(239, 68, 68, 0.25)",
};
