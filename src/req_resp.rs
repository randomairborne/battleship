use crate::Error;
use std::ops::Deref;
use tokio::sync::{
    mpsc::{Receiver as MpscReceiver, Sender as MpscSender},
    oneshot::Sender as OneshotSender,
};

pub fn pair<Req, Resp>() -> (ReqRespClient<Req, Resp>, ReqRespServer<Req, Resp>) {
    let (sender, stream) = tokio::sync::mpsc::channel(128);
    let client = ReqRespClient { sender };
    let server = ReqRespServer { stream };
    (client, server)
}

#[derive(Clone)]
pub struct ReqRespClient<Req, Resp> {
    sender: MpscSender<Request<Req, Resp>>,
}

impl<Req, Resp> ReqRespClient<Req, Resp> {
    pub async fn send(&self, data: Req) -> Result<Resp, Error> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let req = Request {
            inner: data,
            callback: tx,
        };
        self.sender.send(req)?;
        Ok(rx.await?)
    }
}

pub struct ReqRespServer<Req, Resp> {
    stream: MpscReceiver<Request<Req, Resp>>,
}

impl<Req, Resp> ReqRespServer<Req, Resp> {
    pub async fn recv(&mut self) -> Option<Request<Req, Resp>> {
        self.stream.recv().await
    }
}

pub struct Request<Req, Resp> {
    inner: Req,
    callback: OneshotSender<Resp>,
}

impl<Req, Resp> Request<Req, Resp> {
    pub fn respond(self, data: Resp) -> Result<(), Req> {
        self.callback.send(data)
    }
}

impl<Req, Resp> Deref for Request<Req, Resp> {
    type Target = Req;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
