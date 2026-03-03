use std::sync::Arc;
use anyhow::{Context, Result};
use arti_client::{TorClient, TorClientConfig};
use tor_rtcompat::PreferredRuntime;

/// A type alias for our Tor client to make signatures cleaner
pub type AppTorClient = Arc<TorClient<PreferredRuntime>>;

/// Initializes the Tor client by connecting to the network.
///
/// This creates the default configuration, ensuring the necessary state directories are set up,
/// and attempts to bootstrap a connection to the decentralized Tor network.
pub async fn init_tor_client() -> Result<AppTorClient> {
    tracing::info!("Initializing Tor client...");

    // Setup configuration
    // We use the default configuration which automatically handles caching, circuits, and paths
    let config = TorClientConfig::default();

    // Create the Tor client
    // `create_bootstrapped` connects to the network and downloads directory info.
    // This can take a few seconds and requires network access.
    let client = TorClient::create_bootstrapped(config)
        .await
        .context("Failed to bootstrap Tor connection")?;

    tracing::info!("Tor client bootstrap complete.");

    // Return the client wrapped in an Arc for safe sharing across threads
    Ok(Arc::new(client))
}
