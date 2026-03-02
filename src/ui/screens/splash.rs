use dioxus::prelude::*;
use crate::app::Route;
use crate::state::app_state::AppState;
use crate::ui::theme::DARK;
use crate::ui::animations::presets;

/// Animated splash screen.
///
/// Shows the Wraith logo with a pulsing glow, app name, and tagline.
/// After 2 seconds, navigates to onboarding or home.
#[component]
pub fn Splash() -> Element {
    let state = use_context::<Signal<AppState>>();
    let nav = use_navigator();

    // Auto-navigate after 2.5 seconds
    use_future(move || {
        let nav = nav.clone();
        async move {
            tokio::time::sleep(std::time::Duration::from_millis(2500)).await;
            let s = state.read();
            if s.onboarded {
                nav.push(Route::Home {});
            } else {
                nav.push(Route::Onboarding {});
            }
        }
    });

    rsx! {
        div {
            style: "
                width: 100%;
                height: 100vh;
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                background: radial-gradient(circle at center, #0D0A1E 0%, {DARK.bg_primary} 70%);
                position: relative;
                overflow: hidden;
            ",

            // Logo container with glow aura
            div {
                style: "
                    width: 80px;
                    height: 80px;
                    border-radius: 50%;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    animation: {presets::fade_in_up()}, {presets::aura_pulse()};
                    box-shadow: 0 0 40px {DARK.accent_glow};
                ",

                // Ghost logo SVG inline
                svg {
                    width: "60",
                    height: "60",
                    view_box: "0 0 80 80",
                    fill: "none",
                    xmlns: "http://www.w3.org/2000/svg",

                    defs {
                        linearGradient {
                            id: "ghostGrad",
                            x1: "20",
                            y1: "10",
                            x2: "60",
                            y2: "75",
                            stop { offset: "0%", "stop-color": "#9B8AFF" }
                            stop { offset: "100%", "stop-color": "#7C6AF7" }
                        }
                    }
                    path {
                        d: "M40 8C25.088 8 13 20.088 13 35v25c0 2 1 3 2.5 2s3-3 4.5-3 3 2 4.5 3 2.5 0 2.5-2v-3c0-2 1-3 2.5-2s3 3 4.5 3 3-2 4.5-3 2.5 0 2.5 2v3c0 2 1 3 2.5 2s3-3 4.5-3 3 2 4.5 3 2.5 0 2.5-2v-3c0-2 1-3 2.5-2s3 3 4.5 3 3-2 4.5-3 2.5 0 2.5 2V35C67 20.088 54.912 8 40 8z",
                        fill: "url(#ghostGrad)",
                        opacity: "0.95",
                    }
                    ellipse { cx: "31", cy: "34", rx: "5", ry: "6", fill: DARK.bg_primary }
                    ellipse { cx: "49", cy: "34", rx: "5", ry: "6", fill: DARK.bg_primary }
                    ellipse { cx: "33", cy: "32", rx: "2", ry: "2.5", fill: DARK.text_primary, opacity: "0.7" }
                    ellipse { cx: "51", cy: "32", rx: "2", ry: "2.5", fill: DARK.text_primary, opacity: "0.7" }
                }
            }

            // App name
            div {
                style: "
                    margin-top: 24px;
                    font-size: 24px;
                    font-weight: 700;
                    letter-spacing: 0.3em;
                    color: {DARK.text_primary};
                    animation: {presets::fade_in_up_delayed(400)};
                ",
                "WRAITH"
            }

            // Tagline
            div {
                style: "
                    margin-top: 8px;
                    font-size: 13px;
                    color: {DARK.text_secondary};
                    animation: {presets::fade_in_up_delayed(700)};
                ",
                "Anonymous. Encrypted. Yours."
            }
        }
    }
}
