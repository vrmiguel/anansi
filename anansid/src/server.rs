use anansi_core::Message;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{UnixListener, UnixStream},
};

use crate::{Result, SOCKET_PATH};

async fn read_msg<'a, 'b>(
    stream: &'a mut UnixStream,
    mut buf: &'b mut [u8],
) -> Result<Message<'b>> {
    let mut bytes_read_acc = 0;
    loop {
        let bytes_read = stream.read(&mut buf).await?;

        if bytes_read == 0 {
            break;
        }

        tracing::info!("Bytes read: {bytes_read}");
        bytes_read_acc += bytes_read;
    }

    let msg = std::str::from_utf8(&buf[..bytes_read_acc])?;

    Ok(anansi_core::decoder::parse(msg).unwrap())
}

async fn handle_stream(mut stream: UnixStream) -> Result {
    let mut buf = [0; 4096];

    let msg = read_msg(&mut stream, &mut buf).await?;

    tracing::info!("Got message: '{msg}'");

    stream.write(b"OK").await?;

    stream.flush().await?;

    tracing::info!("All done with client");

    Ok(())
}

pub async fn run_server() -> crate::Result {
    let _ = std::fs::remove_file(SOCKET_PATH);
    let listener = UnixListener::bind(SOCKET_PATH)?;

    loop {
        let stream = match listener.accept().await {
            Ok((stream, _)) => stream,
            Err(err) => {
                tracing::error!(
                    "Error accepting connection: {err}. Continuing."
                );
                continue;
            }
        };

        tracing::info!("Accepted connection");

        // TO-DO: handle this in another Tokio task.. once that
        // works
        tokio::spawn(async {
            tracing::info!("Spawning task");
            if let Err(err) = handle_stream(stream).await {
                tracing::error!(
                    "Failed handling an incoming stream: {err}"
                );
            }
        });
    }
}
