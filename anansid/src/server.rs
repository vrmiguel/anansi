use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{UnixListener, UnixStream},
};

use crate::{Result, SOCKET_PATH};

async fn handle_stream(mut stream: UnixStream) -> Result {
    // let (reader, mut writer) = stream.split();

    let mut buf = [0; 4096];
    loop {
        let num_read = stream.read(&mut buf).await?;
        tracing::info!("Bytes read: {num_read}");

        if num_read == 0 {
            break;
        }
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
                eprintln!("Error accepting connection: {err}");
                continue;
            }
        };

        tracing::info!("Accepted connection");

        // Spawn a new task to handle the client
        // tokio::spawn(async move { });
        handle_stream(stream).await?;
    }

    // TODO: know when to close the server
    Ok(())
}
