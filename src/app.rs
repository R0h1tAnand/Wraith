use dioxus::prelude::*;

use crate::state::app_state::AppState;
use crate::ui::screens::{
    splash::Splash,
    onboarding::Onboarding,
    home::Home,
    chat::Chat,
    new_chat::NewChat,
    profile::Profile,
    settings::Settings,
};

/// Application routes.
#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Splash {},
    #[route("/onboarding")]
    Onboarding {},
    #[route("/home")]
    Home {},
    #[route("/chat/:contact_id")]
    Chat { contact_id: String },
    #[route("/new-chat")]
    NewChat {},
    #[route("/profile")]
    Profile {},
    #[route("/settings")]
    Settings {},
}

/// Root application component.
///
/// Provides global state via context and renders the router.
#[component]
pub fn App() -> Element {
    // Provide AppState as a global context signal
    let _state = use_context_provider(|| Signal::new(AppState::new()));

    rsx! {
        style { {include_str!("../assets/global.css")} }
        Router::<Route> {}
    }
}
