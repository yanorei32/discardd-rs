use std::io;
use std::net::SocketAddr;

use clap::Parser;
use tokio::net::TcpListener;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(default_value = "0.0.0.0:9")]
    listen: SocketAddr,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    tracing::info!("Server is ready on {}", cli.listen);

    let listener = TcpListener::bind(cli.listen).await?;

    loop {
        let (mut socket, remote) = listener.accept().await?;
        tracing::info!("{remote} connected!");

        tokio::spawn(async move {
            let (rx, _tx) = socket.split();

            loop {
                if let Err(e) = rx.readable().await {
                    tracing::error!("Read Error [0] {e}");
                    break;
                }

                let mut buf = [0; 4096];

                match rx.try_read(&mut buf) {
                    Ok(0) => break,
                    Ok(_) => {}
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        continue;
                    }
                    Err(e) => {
                        tracing::error!("Read Error [1] {e}");
                        break;
                    }
                }
            }

            tracing::info!("{remote} disconnected!");
        });
    }
}
