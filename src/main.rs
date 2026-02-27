mod app;
mod core;
mod state;
mod ui;

use app::App;

fn main() {
    // Initialize tracing for structured logging
    // On Android, logs go to logcat via the tracing subscriber
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();

    tracing::info!("Starting Wraith v{}", env!("CARGO_PKG_VERSION"));

    // Launch the Dioxus app (mobile platform auto-detected from feature flag)
    dioxus::launch(App);
}
