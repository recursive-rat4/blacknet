/*
 * Copyright (c) 2026 Pavel Vasin
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use core::{
    fmt,
    pin::Pin,
    str::Utf8Error,
    task::{Context, Poll},
};
use futures_util::{SinkExt, StreamExt};
use hyper::{
    Error as HyperError, Request,
    body::{Body, Bytes, Frame, Incoming},
    client::conn::http1,
    rt::{Read, ReadBufCursor, Write},
};
use serde_json::{Error as JsonError, Value, from_str, to_writer_pretty};
use std::io::{Error as IoError, IoSlice, stdout};
use tokio::{
    io::{AsyncRead, AsyncWrite, ReadBuf},
    net::{TcpStream, ToSocketAddrs},
    runtime::Runtime,
};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Error as TungsteniteError, Message},
};

pub struct Client {
    runtime: Runtime,
    endpoint: String,
}

impl Client {
    pub fn new(host: &str, port: u16) -> Result<Self, IoError> {
        Ok(Self {
            runtime: Runtime::new()?,
            endpoint: format!("{}:{}", host, port),
        })
    }

    pub fn get(&self, path: &str) -> Result<String, Error> {
        let request = Request::builder()
            .uri(path)
            .body(String::new())
            .expect("http request");
        self.runtime.block_on(async {
            let stream = HyperStream::connect(&self.endpoint).await?;
            let (mut sender, connection) = http1::handshake(stream).await?;
            self.runtime.spawn(connection);
            let response = sender.send_request(request).await?;
            let mut body = response.into_body();
            let size_hint = body.size_hint().lower() as usize;
            let mut buffer = String::with_capacity(size_hint);
            while let Some(frame) = HyperFrame::new(&mut body).await {
                if let Some(data) = frame?.data_ref() {
                    buffer.push_str(str::from_utf8(data)?)
                }
            }
            Ok(buffer)
        })
    }

    pub fn subscribe(&self, route: &str) -> Result<String, Error> {
        let url = format!("ws://{}/api/v2/websocket", self.endpoint);
        let message = format!("{{\"command\":\"subscribe\",\"route\":\"{route}\"}}");
        self.runtime.block_on(async {
            let (mut stream, _) = connect_async(url).await?;
            stream.send(Message::Text(message.into())).await?;

            while let Some(message) = stream.next().await {
                match message? {
                    Message::Text(message) => {
                        let json = from_str::<Value>(&message)?;
                        to_writer_pretty(stdout(), &json)?;
                        println!();
                    }
                    Message::Binary(_) => return Ok(String::from("Message::Binary")),
                    Message::Close(frame) => {
                        if let Some(frame) = frame {
                            return Ok(format!("Message::Close {}", frame));
                        } else {
                            return Ok(String::from("Message::Close"));
                        }
                    }
                    Message::Ping(_) | Message::Pong(_) | Message::Frame(_) => continue,
                }
            }

            Ok(String::from("WebSocketStream closed"))
        })
    }
}

struct HyperStream {
    tcp_stream: TcpStream,
}

impl HyperStream {
    async fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self, IoError> {
        Ok(Self {
            tcp_stream: TcpStream::connect(addr).await?,
        })
    }
}

impl Read for HyperStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        mut crsr: ReadBufCursor<'_>,
    ) -> Poll<Result<(), IoError>> {
        let n = unsafe {
            let mut buf = ReadBuf::uninit(crsr.as_mut());
            match AsyncRead::poll_read(self.map_unchecked_mut(|x| &mut x.tcp_stream), cx, &mut buf)
            {
                Poll::Ready(Ok(())) => buf.filled().len(),
                x => return x,
            }
        };
        unsafe { crsr.advance(n) }
        Poll::Ready(Ok(()))
    }
}

impl Write for HyperStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, IoError>> {
        AsyncWrite::poll_write(
            unsafe { self.map_unchecked_mut(|x| &mut x.tcp_stream) },
            cx,
            buf,
        )
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), IoError>> {
        AsyncWrite::poll_flush(unsafe { self.map_unchecked_mut(|x| &mut x.tcp_stream) }, cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), IoError>> {
        AsyncWrite::poll_shutdown(unsafe { self.map_unchecked_mut(|x| &mut x.tcp_stream) }, cx)
    }

    fn is_write_vectored(&self) -> bool {
        self.tcp_stream.is_write_vectored()
    }

    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[IoSlice<'_>],
    ) -> Poll<Result<usize, IoError>> {
        AsyncWrite::poll_write_vectored(
            unsafe { self.map_unchecked_mut(|x| &mut x.tcp_stream) },
            cx,
            bufs,
        )
    }
}

struct HyperFrame<'a> {
    incoming: &'a mut Incoming,
}

impl<'a> HyperFrame<'a> {
    const fn new(incoming: &'a mut Incoming) -> Self {
        Self { incoming }
    }
}

impl<'a> Future for HyperFrame<'a> {
    type Output = Option<Result<Frame<Bytes>, HyperError>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Body::poll_frame(unsafe { self.map_unchecked_mut(|x| x.incoming) }, cx)
    }
}

#[derive(Debug)]
pub enum Error {
    Hyper(HyperError),
    Io(IoError),
    Json(JsonError),
    Tungstenite(TungsteniteError),
    Utf8(Utf8Error),
}

impl From<HyperError> for Error {
    fn from(error: HyperError) -> Self {
        Self::Hyper(error)
    }
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Self {
        Self::Io(error)
    }
}

impl From<JsonError> for Error {
    fn from(error: JsonError) -> Self {
        Self::Json(error)
    }
}

impl From<TungsteniteError> for Error {
    fn from(error: TungsteniteError) -> Self {
        Self::Tungstenite(error)
    }
}

impl From<Utf8Error> for Error {
    fn from(error: Utf8Error) -> Self {
        Self::Utf8(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hyper(err) => write!(f, "{err}"),
            Self::Io(err) => write!(f, "{err}"),
            Self::Json(err) => write!(f, "{err}"),
            Self::Tungstenite(err) => write!(f, "{err}"),
            Self::Utf8(err) => write!(f, "{err}"),
        }
    }
}

impl core::error::Error for Error {}
