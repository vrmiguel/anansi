use std::{
    io::{Read, Write},
    net::Shutdown,
    os::unix::net::UnixStream,
    println, write,
};

use anansi_core::Message;

use crate::{Result, SOCKET_PATH};

pub fn send_message(msg: Message<'_>) -> Result<()> {
    println!("connecting to server");
    let mut stream = UnixStream::connect(SOCKET_PATH)?;

    println!("Got connection to server");

    write!(stream, "{msg}")?;

    // Notify server we're done writing.
    stream.shutdown(Shutdown::Write)?;

    {
        println!("Reading from server");

        let mut buf = String::new();
        stream.read_to_string(&mut buf)?;
        println!("Client got response {buf}");
    }

    // Notify server we're done reading as well
    stream.shutdown(Shutdown::Read)?;

    Ok(())
}
