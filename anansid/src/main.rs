mod error;

mod events;
mod runner;
mod server;
mod smallstring;

pub use error::{Error, Result};
use events::EventRegistry;
use server::run_server;

pub const SOCKET_PATH: &str = "/tmp/anansi-socket";

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    tracing_subscriber::fmt().compact().init();

    if let Err(err) = run().await {
        tracing::error!("{err}");
    }
}

async fn run() -> crate::Result {
    let event_registry = EventRegistry::new();

    run_server(event_registry).await
}
