use crate::Error;
use owo_colors::OwoColorize;
use std::fmt::Display;
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub async fn handle_stream(stream: TcpStream) {
    if let Err(e) = run(stream).await {
        eprintln!("{e:?}");
    }
}

async fn run(mut stream: TcpStream) -> Result<(), Error> {
    stream.colorful("test".cyan()).await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    stream.clear().await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    Ok(())
}

pub trait ConnectedTerminal {
    async fn colorful(&mut self, data: impl Display) -> Result<(), Error>;
    async fn clear(&mut self) -> Result<(), Error> {
        self.colorful("\u{1b}[2J").await
    }
}

impl ConnectedTerminal for TcpStream {
    async fn colorful(&mut self, data: impl Display) -> Result<(), Error> {
        self.write_all(data.to_string().as_bytes()).await?;
        Ok(())
    }
}
