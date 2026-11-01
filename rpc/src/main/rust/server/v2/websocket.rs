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

use crate::server::{RpcClient, RpcServer};
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

async fn upgrade(ws: WebSocketUpgrade, State(rpc_server): State<Arc<RpcServer>>) -> Response {
    ws.on_upgrade(|socket| on_socket(socket, rpc_server))
}

async fn on_socket(socket: WebSocket, rpc_server: Arc<RpcServer>) {
    let (rpc_client, eventer) = rpc_server.create_client();
    let (sink, stream) = socket.split();
    tokio::spawn(sender(sink, eventer));
    tokio::spawn(receiver(stream, rpc_client, rpc_server));
}

async fn receiver(
    mut stream: SplitStream<WebSocket>,
    rpc_client: RpcClient,
    rpc_server: Arc<RpcServer>,
) {
    while let Some(Ok(message)) = stream.next().await {
        match message {
            Message::Text(utf8) => {
                if on_text(&utf8, &rpc_client, &rpc_server).await.is_break() {
                    break;
                }
            }
            Message::Binary(_) => break,
            Message::Ping(_) | Message::Pong(_) => continue,
            Message::Close(_) => break,
        }
    }
}

async fn on_text(
    utf8: &Utf8Bytes,
    rpc_client: &RpcClient,
    rpc_server: &RpcServer,
) -> ControlFlow<()> {
    match from_str::<Value>(utf8) {
        Ok(Value::Object(object)) => on_object(&object, rpc_client, rpc_server).await,
        _ => ControlFlow::Break(()),
    }
}

async fn on_object(
    request: &Map<String, Value>,
    rpc_client: &RpcClient,
    rpc_server: &RpcServer,
) -> ControlFlow<()> {
    let Some(Value::String(command)) = request.get("command") else {
        return ControlFlow::Break(());
    };
    match command.as_str() {
        "subscribe" => on_subscribe(request, rpc_client, rpc_server).await,
        "unsubscribe" => on_unsubscribe(request, rpc_client, rpc_server).await,
        _ => ControlFlow::Break(()),
    }
}

async fn on_subscribe(
    request: &Map<String, Value>,
    rpc_client: &RpcClient,
    rpc_server: &RpcServer,
) -> ControlFlow<()> {
    let Some(Value::String(route)) = request.get("route") else {
        return ControlFlow::Break(());
    };
    match route.as_str() {
        "block" => {
            rpc_server.subscribe_block(rpc_client).await;
            ControlFlow::Continue(())
        }
        "txpool" => {
            rpc_server.subscribe_txpool(rpc_client).await;
            ControlFlow::Continue(())
        }
        "wallet" => on_subscribe_wallet(request, rpc_client, rpc_server).await,
        _ => ControlFlow::Break(()),
    }
}

async fn on_subscribe_wallet(
    request: &Map<String, Value>,
    rpc_client: &RpcClient,
    rpc_server: &RpcServer,
) -> ControlFlow<()> {
    let Some(Value::String(address)) = request.get("address") else {
        return ControlFlow::Break(());
    };
    let Ok(public_key) = rpc_server.address_codec().decode(address) else {
        return ControlFlow::Break(());
    };
    rpc_server.subscribe_wallet(public_key, rpc_client).await;
    ControlFlow::Continue(())
}

async fn on_unsubscribe(
    request: &Map<String, Value>,
    rpc_client: &RpcClient,
    rpc_server: &RpcServer,
) -> ControlFlow<()> {
    let Some(Value::String(route)) = request.get("route") else {
        return ControlFlow::Break(());
    };
    match route.as_str() {
        "block" => {
            rpc_server.unsubscribe_block(rpc_client).await;
            ControlFlow::Continue(())
        }
        "txpool" => {
            rpc_server.unsubscribe_txpool(rpc_client).await;
            ControlFlow::Continue(())
        }
        "wallet" => on_unsubscribe_wallet(request, rpc_client, rpc_server).await,
        _ => ControlFlow::Break(()),
    }
}

async fn on_unsubscribe_wallet(
    request: &Map<String, Value>,
    rpc_client: &RpcClient,
    rpc_server: &RpcServer,
) -> ControlFlow<()> {
    let Some(Value::String(address)) = request.get("address") else {
        return ControlFlow::Break(());
    };
    let Ok(public_key) = rpc_server.address_codec().decode(address) else {
        return ControlFlow::Break(());
    };
    rpc_server.unsubscribe_wallet(public_key, rpc_client).await;
    ControlFlow::Continue(())
}

async fn sender(mut sink: SplitSink<WebSocket, Message>, mut receiver: mpsc::Receiver<Message>) {
    while let Some(message) = receiver.recv().await {
        if sink.send(message).await.is_err() {
            break;
        }
    }
}

pub fn routes() -> Router<Arc<RpcServer>> {
    Router::new().route("/api/v2/websocket", any(upgrade))
}
