use clap::Parser;

mod cli;
mod error;

mod client;

use cli::Args;
use client::send_message;
pub use error::{Error, Result};

// TODO: move constant to core
pub const SOCKET_PATH: &str = "/tmp/anansi-socket";

fn main() {
    let args = cli::Args::parse();

    tracing_subscriber::fmt().compact().init();

    if let Err(err) = run(args) {
        tracing::error!("{err}");
    }
}

fn run(args: Args) -> crate::Result {
    match args {
        Args::On {
            message: _,
            action: _,
        } => {}
        Args::Send { message } => {
            send_message(&message)?;
        }
    }

    Ok(())
}
