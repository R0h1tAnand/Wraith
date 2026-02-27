use dioxus::prelude::*;
use crate::app::Route;
use crate::state::app_state::AppState;
use crate::state::actions;
use crate::ui::theme::DARK;
use crate::ui::animations::presets;

/// Onboarding screen — 3-step first-time identity generation flow.
#[component]
pub fn Onboarding() -> Element {
    let mut state = use_context::<Signal<AppState>>();
    let nav = use_navigator();
    let mut step = use_signal(|| 0u32); // 0=Welcome, 1=Generating, 2=Backup
    let mut generated_key = use_signal(|| String::new());
    let mut copied = use_signal(|| false);

    rsx! {
        div {
            style: "
                width: 100%;
                height: 100vh;
                background: {DARK.bg_primary};
                display: flex;
                flex-direction: column;
                overflow: hidden;
            ",

            match *step.read() {
                0 => rsx! { StepWelcome { on_continue: move |_| step.set(1) } },
                1 => rsx! { StepGenerating {
                    state: state,
                    generated_key: generated_key,
                    step: step,
                } },
                _ => rsx! { StepBackup {
                    public_key: generated_key.read().clone(),
                    copied: copied,
                    on_copy: move |_| copied.set(true),
                    on_continue: move |_| {
                        nav.push(Route::Home {});
                    },
                } },
            }
        }
    }
}

/// Step 1 — Welcome page.
#[component]
fn StepWelcome(on_continue: EventHandler<()>) -> Element {
    rsx! {
        div {
            style: "
                flex: 1;
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                padding: 32px;
                animation: {presets::fade_in_up()};
            ",

            // Ghost icon
            div {
                style: "
                    width: 100px;
                    height: 100px;
                    border-radius: 50%;
                    background: {DARK.bg_tertiary};
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    margin-bottom: 32px;
                    box-shadow: 0 0 40px {DARK.accent_glow};
                ",
                span { style: "font-size: 48px;", "👻" }
            }

            h1 {
                style: "
                    font-size: 28px;
                    font-weight: 700;
                    color: {DARK.text_primary};
                    margin-bottom: 12px;
                    text-align: center;
                ",
                "Welcome to Wraith"
            }

            p {
                style: "
                    font-size: 15px;
                    color: {DARK.text_secondary};
                    text-align: center;
                    line-height: 1.6;
                    max-width: 320px;
                    margin-bottom: 48px;
                ",
                "No phone number. No email. No servers tracking you. Your identity is a cryptographic keypair — nothing more."
            }

            button {
                class: "btn-primary",
                onclick: move |_| on_continue.call(()),
                "Get Started"
            }
        }
    }
}

/// Step 2 — Identity generation with spinner.
#[component]
fn StepGenerating(
    state: Signal<AppState>,
    generated_key: Signal<String>,
    step: Signal<u32>,
) -> Element {
    let mut state = state;
    let mut generated_key = generated_key;
    let mut step = step;

    // Simulate keypair generation
    use_future(move || async move {
        // Brief delay for animation
        tokio::time::sleep(std::time::Duration::from_millis(1200)).await;

        // Generate the identity
        actions::action_generate_identity(&mut state);

        // Read the key
        let key = state.read().identity.as_ref()
            .map(|id| id.public_key_hex())
            .unwrap_or_default();
        generated_key.set(key);

        // Short pause to show checkmark
        tokio::time::sleep(std::time::Duration::from_millis(800)).await;
        step.set(2);
    });

    rsx! {
        div {
            style: "
                flex: 1;
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                padding: 32px;
                animation: slideLeft 300ms cubic-bezier(0.16, 1, 0.3, 1) both;
            ",

            // Spinner
            div {
                style: "
                    width: 64px;
                    height: 64px;
                    border: 3px solid {DARK.bg_tertiary};
                    border-top-color: {DARK.accent_primary};
                    border-radius: 50%;
                    animation: {presets::spin()};
                    margin-bottom: 24px;
                ",
            }

            p {
                style: "
                    font-size: 17px;
                    font-weight: 600;
                    color: {DARK.text_primary};
                    margin-bottom: 8px;
                ",
                "Generating your identity..."
            }

            p {
                style: "
                    font-size: 13px;
                    color: {DARK.text_secondary};
                ",
                "Creating Ed25519 + X25519 keypair"
            }
        }
    }
}

/// Step 3 — Backup warning with full key display.
#[component]
fn StepBackup(
    public_key: String,
    copied: Signal<bool>,
    on_copy: EventHandler<()>,
    on_continue: EventHandler<()>,
) -> Element {
    // Truncate for pill display
    let short_key = if public_key.len() > 20 {
        format!("{}...{}", &public_key[..16], &public_key[public_key.len()-8..])
    } else {
        public_key.clone()
    };

    rsx! {
        div {
            style: "
                flex: 1;
                display: flex;
                flex-direction: column;
                padding: 32px;
                animation: slideLeft 300ms cubic-bezier(0.16, 1, 0.3, 1) both;
            ",

            // Checkmark success
            div {
                style: "
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    margin-top: 32px;
                    margin-bottom: 24px;
                ",
                div {
                    style: "
                        width: 64px;
                        height: 64px;
                        border-radius: 50%;
                        background: rgba(74, 222, 128, 0.1);
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        animation: {presets::scale_in()};
                    ",
                    span { style: "font-size: 32px;", "✓" }
                }
            }

            h2 {
                style: "
                    font-size: 22px;
                    font-weight: 700;
                    color: {DARK.text_primary};
                    text-align: center;
                    margin-bottom: 8px;
                ",
                "Identity Created"
            }

            p {
                style: "
                    font-size: 14px;
                    color: {DARK.text_secondary};
                    text-align: center;
                    margin-bottom: 24px;
                ",
                "This IS your identity. No username. No password."
            }

            // Key display card
            div {
                style: "
                    background: {DARK.bg_secondary};
                    border: 1px solid {DARK.border_subtle};
                    border-radius: 16px;
                    padding: 16px;
                    margin-bottom: 12px;
                ",

                p {
                    style: "
                        font-size: 11px;
                        font-weight: 600;
                        color: {DARK.text_secondary};
                        text-transform: uppercase;
                        letter-spacing: 0.05em;
                        margin-bottom: 8px;
                    ",
                    "Your Public Key"
                }

                p {
                    style: "
                        font-family: 'Cascadia Code', 'Fira Code', monospace;
                        font-size: 12px;
                        color: {DARK.accent_primary};
                        word-break: break-all;
                        line-height: 1.6;
                    ",
                    "{public_key}"
                }
            }

            // Copy button
            button {
                class: "btn-secondary",
                style: "width: 100%; margin-bottom: 24px;",
                onclick: move |_| on_copy.call(()),
                if *copied.read() { "✓ Copied!" } else { "📋 Copy Key" }
            }

            // Info card
            div {
                style: "
                    background: rgba(124, 106, 247, 0.06);
                    border: 1px solid rgba(124, 106, 247, 0.15);
                    border-radius: 16px;
                    padding: 16px;
                    margin-bottom: 32px;
                ",
                p {
                    style: "
                        font-size: 13px;
                        color: {DARK.text_secondary};
                        line-height: 1.6;
                    ",
                    "🔑 Anyone with your key can message you. Share it to connect."
                }
            }

            // Spacer
            div { style: "flex: 1;" }

            // Continue button
            button {
                class: "btn-primary",
                onclick: move |_| on_continue.call(()),
                "I understand — Enter Wraith"
            }
        }
    }
}
