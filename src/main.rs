use clap::Parser;

mod cli;
mod error;

mod client;
mod server;

use cli::Args;
use client::send_message;
pub use error::{Error, Result};
use server::run_server;

pub const SOCKET_PATH: &str = "/tmp/anansi-socket";

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();

    tracing_subscriber::fmt().compact().init();

    if let Err(err) = run(args).await {
        tracing::error!("{err}");
    }
}

async fn run(args: Args) -> crate::Result {
    match args {
        Args::On { message, action } => {}
        Args::Send { message } => {
            send_message(&message)?;
        }
        Args::Start {} => {
            run_server().await?;
        }
    }

    Ok(())
}
