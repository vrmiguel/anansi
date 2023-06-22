use std::eprintln;

use anansi_core::Message;
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

    if let Err(err) = run(args) {
        eprintln!("Error: {err}");
        // TODO: return non-zero status
    }
}

fn run(args: Args) -> crate::Result {
    let msg = match &args {
        Args::On { channel, run } => Message::On {
            channel_name: &channel,
            run,
        },
        Args::Send { channel } => Message::Send {
            channel_name: &channel,
        },
    };

    send_message(msg)?;

    Ok(())
}
