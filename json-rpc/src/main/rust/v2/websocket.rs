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

use crate::{RPCServer, Subscriber};
use axum::{
    Router,
    extract::{
        State,
        ws::{Message, Utf8Bytes, WebSocket, WebSocketUpgrade},
    },
    response::Response,
    routing::any,
};
use core::ops::ControlFlow;
use futures_util::{
    sink::SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
};
use serde_json::{Map, from_str, value::Value};
use std::sync::Arc;
use tokio::sync::mpsc;

async fn upgrade(ws: WebSocketUpgrade, State(rpc_server): State<Arc<RPCServer>>) -> Response {
    ws.on_upgrade(|socket| on_socket(socket, rpc_server))
}

async fn on_socket(socket: WebSocket, rpc_server: Arc<RPCServer>) {
    let (subscriber, eventer) = rpc_server.create_subscriber();
    let (sink, stream) = socket.split();
    tokio::spawn(sender(sink, eventer));
    tokio::spawn(receiver(stream, subscriber, rpc_server));
}

async fn receiver(
    mut stream: SplitStream<WebSocket>,
    subscriber: Subscriber,
    rpc_server: Arc<RPCServer>,
) {
    while let Some(Ok(message)) = stream.next().await {
        match message {
            Message::Text(utf8) => {
                if on_text(&utf8, &subscriber, &rpc_server).is_break() {
                    break;
                }
            }
            Message::Binary(_) => break,
            Message::Ping(_) | Message::Pong(_) => continue,
            Message::Close(_) => break,
        }
    }
}

fn on_text(utf8: &Utf8Bytes, subscriber: &Subscriber, rpc_server: &RPCServer) -> ControlFlow<()> {
    match from_str::<Value>(utf8) {
        Ok(Value::Object(object)) => on_object(&object, subscriber, rpc_server),
        _ => ControlFlow::Break(()),
    }
}

fn on_object(
    request: &Map<String, Value>,
    subscriber: &Subscriber,
    rpc_server: &RPCServer,
) -> ControlFlow<()> {
    let Some(Value::String(command)) = request.get("command") else {
        return ControlFlow::Break(());
    };
    match command.as_str() {
        "subscribe" => on_subscribe(request, subscriber, rpc_server),
        "unsubscribe" => on_unsubscribe(request, subscriber, rpc_server),
        _ => ControlFlow::Break(()),
    }
}

fn on_subscribe(
    request: &Map<String, Value>,
    subscriber: &Subscriber,
    rpc_server: &RPCServer,
) -> ControlFlow<()> {
    let Some(Value::String(route)) = request.get("route") else {
        return ControlFlow::Break(());
    };
    match route.as_str() {
        "block" => {
            rpc_server.subscribe_block(subscriber);
            ControlFlow::Continue(())
        }
        "txpool" => {
            rpc_server.subscribe_txpool(subscriber);
            ControlFlow::Continue(())
        }
        "wallet" => {
            rpc_server.subscribe_wallet(subscriber);
            ControlFlow::Continue(())
        }
        _ => ControlFlow::Break(()),
    }
}

fn on_unsubscribe(
    request: &Map<String, Value>,
    subscriber: &Subscriber,
    rpc_server: &RPCServer,
) -> ControlFlow<()> {
    let Some(Value::String(route)) = request.get("route") else {
        return ControlFlow::Break(());
    };
    match route.as_str() {
        "block" => {
            rpc_server.unsubscribe_block(subscriber);
            ControlFlow::Continue(())
        }
        "txpool" => {
            rpc_server.unsubscribe_txpool(subscriber);
            ControlFlow::Continue(())
        }
        "wallet" => {
            rpc_server.unsubscribe_wallet(subscriber);
            ControlFlow::Continue(())
        }
        _ => ControlFlow::Break(()),
    }
}

async fn sender(mut sink: SplitSink<WebSocket, Message>, mut receiver: mpsc::Receiver<Message>) {
    while let Some(message) = receiver.recv().await {
        if sink.send(message).await.is_err() {
            break;
        }
    }
}

pub fn routes() -> Router<Arc<RPCServer>> {
    Router::new().route("/api/v2/websocket", any(upgrade))
}
