use anansi_core::Message;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{UnixListener, UnixStream},
};

use crate::{events::EventRegistry, Result, SOCKET_PATH};

async fn read_msg<'a, 'b>(
    stream: &'a mut UnixStream,
    buf: &'b mut [u8],
) -> Result<Message<'b>> {
    let mut bytes_read_acc = 0;
    loop {
        let bytes_read = stream.read(buf).await?;

        if bytes_read == 0 {
            break;
        }

        tracing::info!("Bytes read: {bytes_read}");
        bytes_read_acc += bytes_read;
    }

    let msg = std::str::from_utf8(&buf[..bytes_read_acc])?;

    // TODO: error treatment
    Ok(anansi_core::decoder::parse(msg).unwrap())
}

async fn handle_stream(
    mut stream: UnixStream,
    registry: EventRegistry,
) -> Result {
    let mut buf = [0; 1024];

    // Read and parse the client's message
    let msg = {
        let msg = read_msg(&mut stream, &mut buf).await?;
        tracing::info!("Got message: '{msg}'");

        msg
    };

    let response = registry.handle_message(msg).await;
    stream.write_all(response.as_bytes()).await?;

    stream.flush().await?;

    Ok(())
}

pub async fn run_server(
    registry: EventRegistry,
) -> crate::Result {
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

        let registry_for_task = registry.clone();
        tokio::spawn(async move {
            tracing::info!("Spawning task");
            if let Err(err) =
                handle_stream(stream, registry_for_task).await
            {
                tracing::error!(
                    "Failed handling an incoming stream: {err}"
                );
            }
        });
    }
}
