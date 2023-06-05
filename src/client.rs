use std::{
    io::{Read, Write},
    net::Shutdown,
    os::unix::net::UnixStream,
};

use crate::{Result, SOCKET_PATH};

pub fn send_message(msg: &str) -> Result<()> {
    tracing::info!("connecting to server");
    let mut stream = UnixStream::connect(SOCKET_PATH)?;

    tracing::info!("Got connection to server");

    stream.write_all(msg.as_bytes())?;
    // Notify server we're done writing.
    stream.shutdown(Shutdown::Write)?;

    {
        tracing::info!("Reading shit");

        let mut buf = String::new();
        stream.read_to_string(&mut buf)?;
        tracing::info!("Client got response {buf}");
    }

    // Notify server we're done reading as well
    stream.shutdown(Shutdown::Read)?;

    Ok(())
}
