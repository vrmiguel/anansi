use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{UnixListener, UnixStream},
};

use crate::{Result, SOCKET_PATH};

async fn handle_stream(mut stream: UnixStream) -> Result {
    let mut buf = [0; 4096];
    loop {
        let num_read = stream.read(&mut buf).await?;

        if num_read == 0 {
            break;
        }

        tracing::info!("Bytes read: {num_read}");
    }

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

    // TODO: know when to close the server
    Ok(())
}
