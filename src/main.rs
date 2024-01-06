#![allow(clippy::module_name_repetitions)]
mod board;
mod cell;
mod error;
mod req_resp;
mod ship;
mod stream;
mod ui;
mod util;

use std::collections::HashMap;
use std::io::Stdout;
use std::net::SocketAddr;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::select;
use tokio::task::JoinSet;

use cell::Cell;

use crate::req_resp::ReqRespClient;
pub use error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 1967));
    let listener = TcpListener::bind(addr).await?;
    let mut tasks: JoinSet<()> = JoinSet::new();
    let state = State::new();
    loop {
        let (stream, _address) = match select! {
            sock = listener.accept() => sock,
            _ = vss::shutdown_signal() => break,
        } {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Couldn't get socket: {e}");
                continue;
            }
        };
        tasks.spawn(stream::handle_stream(stream));
    }
    while tasks.join_next().await.is_some() {}
    Ok(())
}

pub struct State {
    pending_rooms: Arc<Mutex<HashMap<String, ReqRespClient<String, String>>>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            pending_rooms: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
