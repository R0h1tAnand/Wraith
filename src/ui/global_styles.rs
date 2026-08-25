use dioxus::prelude::*;
use crate::ui::theme::DARK;

/// Injects global CSS keyframes, resets, and design tokens into the DOM.
///
/// Mount this component once at the app root (e.g. inside `App`).
#[component]
pub fn GlobalStyles() -> Element {
    rsx! {
        style {
            r#"
                /* ── Reset & Base ───────────────────────────────────── */
                *, *::before, *::after {{
                    box-sizing: border-box;
                    margin: 0;
                    padding: 0;
                    -webkit-tap-highlight-color: transparent;
                    -webkit-font-smoothing: antialiased;
                    -moz-osx-font-smoothing: grayscale;
                }}

                body {{
                    font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Display',
                                 'Segoe UI', Roboto, 'Helvetica Neue', sans-serif;
                    background: {bg};
                    color: {text};
                    overflow: hidden;
                    user-select: none;
                    -webkit-user-select: none;
                    letter-spacing: -0.01em;
                }}

                /* ── Input styling ──────────────────────────────────── */
                .input-field {{
                    background: {glass_bg};
                    border: 1px solid {glass_border};
                    color: {text};
                    outline: none;
                    transition: border-color 200ms ease, box-shadow 200ms ease;
                    backdrop-filter: {backdrop};
                    -webkit-backdrop-filter: {backdrop};
                }}

                .input-field:focus {{
                    border-color: {accent}44;
                    box-shadow: 0 0 0 3px {accent}15, inset 0 0 12px {accent}08;
                }}

                .input-field::placeholder {{
                    color: {text_tertiary};
                }}

                /* ── Scrollbar ──────────────────────────────────────── */
                ::-webkit-scrollbar {{ width: 3px; }}
                ::-webkit-scrollbar-track {{ background: transparent; }}
                ::-webkit-scrollbar-thumb {{
                    background: rgba(255, 255, 255, 0.08);
                    border-radius: 4px;
                }}

                /* ── Keyframes ──────────────────────────────────────── */
                @keyframes wraith-fade-in-up {{
                    from {{ opacity: 0; transform: translateY(16px); }}
                    to   {{ opacity: 1; transform: translateY(0); }}
                }}

                @keyframes wraith-glass-appear {{
                    from {{ opacity: 0; transform: scale(0.96); }}
                    to   {{ opacity: 1; transform: scale(1); }}
                }}

                @keyframes wraith-slide-up {{
                    from {{ transform: translateY(100%); }}
                    to   {{ transform: translateY(0); }}
                }}

                @keyframes wraith-slide-down {{
                    from {{ transform: translateY(0); }}
                    to   {{ transform: translateY(100%); }}
                }}

                @keyframes wraith-fade-in {{
                    from {{ opacity: 0; }}
                    to   {{ opacity: 1; }}
                }}

                @keyframes wraith-breathe {{
                    0%, 100% {{ opacity: 0.6; transform: scale(1); }}
                    50%      {{ opacity: 1;   transform: scale(1.05); }}
                }}

                @keyframes wraith-scale-tap {{
                    0%   {{ transform: scale(1); }}
                    40%  {{ transform: scale(0.92); }}
                    100% {{ transform: scale(1); }}
                }}

                @keyframes wraith-shimmer {{
                    0%   {{ background-position: -200% 0; }}
                    100% {{ background-position: 200% 0; }}
                }}

                @keyframes wraith-toast-enter {{
                    from {{ opacity: 0; transform: translateY(-20px) scale(0.95); }}
                    to   {{ opacity: 1; transform: translateY(0) scale(1); }}
                }}

                @keyframes wraith-toast-exit {{
                    from {{ opacity: 1; transform: translateY(0) scale(1); }}
                    to   {{ opacity: 0; transform: translateY(-20px) scale(0.95); }}
                }}

                @keyframes wraith-spin {{
                    from {{ transform: rotate(0deg); }}
                    to   {{ transform: rotate(360deg); }}
                }}

                @keyframes wraith-glow-pulse {{
                    0%, 100% {{ box-shadow: 0 0 8px rgba(124,106,247,0.2); }}
                    50%      {{ box-shadow: 0 0 20px rgba(124,106,247,0.45); }}
                }}
            "#,
            bg = DARK.bg_primary,
            text = DARK.text_primary,
            glass_bg = DARK.glass_bg,
            glass_border = DARK.glass_border,
            backdrop = DARK.glass_backdrop,
            accent = DARK.accent_primary,
            text_tertiary = DARK.text_tertiary,
        }
    }
}
