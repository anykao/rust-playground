extern crate futures;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;

use std::io;
use std::str;
use tokio_core::io::{Codec, EasyBuf, Io, Framed};
use tokio_proto::pipeline::ServerProto;
use tokio_service::Service;
use futures::{future, Future, BoxFuture};
use tokio_proto::TcpServer;

pub struct LineCodec;

impl Codec for LineCodec {
    type In = String;
    type Out = String;

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Self::In>> {
        if let Some(i) = buf.as_slice().iter().position(|&b| b == b'\n') {
            let line = buf.drain_to(i);
            buf.drain_to(1);
            match str::from_utf8(line.as_slice()) {
                Ok(s) => Ok(Some(s.to_string())),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "invalid UTF-8")),
            }

        } else {
            Ok(None)
        }
    }

    fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) -> io::Result<()> {
        buf.extend(msg.as_bytes());
        buf.push(b'\n');
        Ok(())
    }
}

pub struct LineProto;
impl<T: Io + 'static> ServerProto<T> for LineProto {
    type Request = String;
    type Response = String;
    type Transport = Framed<T, LineCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;
    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(LineCodec))
    }
}

pub struct Echo;

impl Service for Echo {
    /// Requests handled by the service.
    type Request = String;

    /// Responses given by the service.
    type Response = String;

    /// Errors produced by the service.
    type Error = io::Error;

    /// The future response value.
    type Future = BoxFuture<Self::Response, Self::Error>;

    /// Process the request and return the response asynchronously.
    fn call(&self, req: Self::Request) -> Self::Future {
        future::ok(req).boxed()
    }
}

fn main() {

    // Specify the localhost address
    let addr = "0.0.0.0:12345".parse().unwrap();

    // The builder requires a protocol and an address
    let server = TcpServer::new(LineProto, addr);

    // We provide a way to *instantiate* the service for each new
    // connection; here, we just immediately return a new instance.
    server.serve(|| Ok(Echo));
}
