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

use axum::{
    Router,
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::Response,
    routing::any,
};
use blacknet_network::node::Node;
use futures_util::stream::{SplitSink, SplitStream, StreamExt};
use std::sync::Arc;

async fn upgrade(ws: WebSocketUpgrade, State(node): State<Arc<Node>>) -> Response {
    ws.on_upgrade(|socket| handle(socket, node))
}

async fn handle(socket: WebSocket, node: Arc<Node>) {
    let (sink, stream) = socket.split();
    tokio::spawn(sender(sink));
    tokio::spawn(receiver(stream, node));
}

async fn receiver(mut stream: SplitStream<WebSocket>, _node: Arc<Node>) {
    while let Some(Ok(message)) = stream.next().await {
        #[expect(clippy::match_single_binding)]
        match message {
            _ => todo!(),
        }
    }
}

async fn sender(_sink: SplitSink<WebSocket, Message>) {
    //TODO
}

pub fn routes() -> Router<Arc<Node>> {
    Router::new().route("/api/v2/websocket", any(upgrade))
}
