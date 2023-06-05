#[derive(Debug, clap::Parser)]
#[command(
    name = "msg-cli",
    version = "1.0",
    author = "Your Name",
    about = "A command-line tool for message passing and IPC"
)]
pub enum Args {
    #[clap(name = "start", about = "start the Anansi server")]
    Start {},
    #[clap(
        name = "on",
        about = "Listen for a specific message and perform an action"
    )]
    On {
        #[clap(help = "The message to listen for")]
        message: String,
        #[clap(
            help = "The action to perform when the message is received"
        )]
        action: String,
    },
    #[clap(
        name = "send",
        about = "Send a message to trigger an action"
    )]
    Send {
        #[clap(help = "The message to send")]
        message: String,
    },
}
