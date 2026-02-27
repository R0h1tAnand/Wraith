/// Design system tokens for Wraith.
///
/// All colors, spacing, typography, and border radius values
/// are centralized here for consistency.

/// Color theme definition.
pub struct Theme {
    // Backgrounds
    pub bg_primary: &'static str,
    pub bg_secondary: &'static str,
    pub bg_tertiary: &'static str,
    pub bg_input: &'static str,

    // Accents
    pub accent_primary: &'static str,
    pub accent_glow: &'static str,
    pub accent_green: &'static str,
    pub accent_red: &'static str,
    pub accent_amber: &'static str,
    pub accent_orange: &'static str,

    // Text
    pub text_primary: &'static str,
    pub text_secondary: &'static str,
    pub text_tertiary: &'static str,

    // Borders
    pub border_subtle: &'static str,
    pub border_active: &'static str,
}

/// Dark theme — default and only theme.
pub const DARK: Theme = Theme {
    bg_primary: "#0A0A0F",
    bg_secondary: "#111118",
    bg_tertiary: "#1A1A24",
    bg_input: "#16161F",

    accent_primary: "#7C6AF7",
    accent_glow: "#7C6AF733",
    accent_green: "#4ADE80",
    accent_red: "#F87171",
    accent_amber: "#FBBF24",
    accent_orange: "#FB923C",

    text_primary: "#F0F0FF",
    text_secondary: "#8888AA",
    text_tertiary: "#44445A",

    border_subtle: "#1E1E2E",
    border_active: "#7C6AF755",
};

/// Typography scale (in pixels).
pub mod typography {
    pub const SIZE_XS: &str = "11px";
    pub const SIZE_SM: &str = "13px";
    pub const SIZE_BASE: &str = "15px";
    pub const SIZE_MD: &str = "17px";
    pub const SIZE_LG: &str = "20px";
    pub const SIZE_XL: &str = "24px";
    pub const SIZE_2XL: &str = "32px";

    pub const WEIGHT_REGULAR: &str = "400";
    pub const WEIGHT_MEDIUM: &str = "500";
    pub const WEIGHT_SEMIBOLD: &str = "600";
    pub const WEIGHT_BOLD: &str = "700";

    pub const FONT_FAMILY: &str = "'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif";
    pub const FONT_MONO: &str = "'JetBrains Mono', 'Fira Code', 'Cascadia Code', monospace";
}

/// Spacing scale (base-4 system).
pub mod spacing {
    pub const XS: &str = "4px";
    pub const SM: &str = "8px";
    pub const MD: &str = "12px";
    pub const BASE: &str = "16px";
    pub const LG: &str = "20px";
    pub const XL: &str = "24px";
    pub const XXL: &str = "32px";
    pub const XXXL: &str = "48px";
    pub const XXXXL: &str = "64px";
}

/// Border radius constants.
pub mod radius {
    pub const SMALL: &str = "8px";
    pub const MEDIUM: &str = "16px";
    pub const LARGE: &str = "24px";
    pub const FULL: &str = "9999px";
}
