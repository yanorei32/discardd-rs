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
    let cli = Cli::parse();

    println!("Listen on: {}", cli.listen);

    let listener = TcpListener::bind(cli.listen).await?;

    loop {
        let (socket, remote) = listener.accept().await?;
        println!("{remote} connected!");

        tokio::spawn(async move {
            loop {
                socket.readable().await.expect("sockerr");

                let mut buf = [0; 4096];

                match socket.try_read(&mut buf) {
                    Ok(0) => break,
                    Ok(_) => {}
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        continue;
                    }
                    Err(e) => Err(e).expect("readerr"),
                }
            }

            println!("{remote} disconnected!");
        });
    }
}
