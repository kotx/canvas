use std::{sync::Arc, time::Duration};

use axum::{
	extract::{
		ws::{Message, WebSocket},
		State, WebSocketUpgrade,
	},
	routing::get,
	Router,
};
use futures::{
	stream::{SplitSink, SplitStream},
	SinkExt, StreamExt,
};

use crate::canvas::Canvas;

#[derive(Clone)]
struct AppState {
	canvas: Canvas,
	diff: Canvas,
}

async fn handle_ws(mut socket: WebSocket, state: Arc<AppState>) {
	let (mut sender, mut receiver) = socket.split();

	tokio::spawn(write(sender));
	tokio::spawn(read(receiver));
}

async fn read(mut receiver: SplitStream<WebSocket>) {
	while let Some(Ok(msg)) = receiver.next().await {
		dbg!(msg);
	}
}

async fn write(mut sender: SplitSink<WebSocket, Message>) {
	while let Ok(()) = sender.send(Message::Text("hello".to_string())).await {
		tokio::time::sleep(Duration::from_secs(1)).await;
	}
}

pub async fn launch() {
	let r = 1024;
	let canvas = Canvas::new(r, r);

	let app = Router::new()
		.route("/", get(|| async { "something should be here" }))
		.route(
			"/ws",
			get(
				|ws: WebSocketUpgrade, State(state): State<Arc<AppState>>| async {
					ws.on_upgrade(|socket| handle_ws(socket, state))
				},
			),
		)
		.with_state(Arc::new(AppState {
			canvas: canvas.clone(),
			diff: canvas,
		}));

	axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
		.serve(app.into_make_service())
		.await
		.unwrap();
}
