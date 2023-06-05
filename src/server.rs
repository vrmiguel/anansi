use tokio::{
    io::{
        AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader,
    },
    net::{TcpStream, UnixListener, UnixStream},
};

use crate::{Result, SOCKET_PATH};

async fn handle_stream(mut stream: UnixStream) -> Result {
    // let (reader, mut writer) = stream.split();

    tracing::info!("??");

    let mut buf = [0; 4096];
    loop {
        tracing::info!("???");

        let num_read = stream.read(&mut buf).await?;
        tracing::info!("???, {num_read}");

        if num_read == 0 {
            break;
        }
        // let buf = &buf[..num_read];
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
