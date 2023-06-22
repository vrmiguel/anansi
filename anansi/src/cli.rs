#[derive(Debug, clap::Parser)]
#[command(
    name = "anansi",
    author = "Vinícius R. Miguel",
    about = "Client for the anansi server"
)]
pub enum Args {
    #[clap(
        name = "on",
        about = "Listen for a specific message and perform an action"
    )]
    On {
        #[clap(help = "The message to listen for")]
        channel: String,
        #[clap(
            help = "The action to perform when the message is received"
        )]
        run: String,
    },
    #[clap(
        name = "send",
        about = "Send a message to trigger an action"
    )]
    Send {
        #[clap(help = "The message to send")]
        channel: String,
    },
}
