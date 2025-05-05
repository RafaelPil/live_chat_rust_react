use axum::{
    extract::State,
    response::Html, routing::get, Json, Router
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tower_http::cors::{CorsLayer, Any};
use http::Method;

#[derive(Deserialize, Serialize, Clone)]
struct Message {
    name: String,
    text: String,
}

type MessageList = Arc<Mutex<Vec<Message>>>;

async fn handle_message(
    State(messages): State<MessageList>,
    Json(msg): Json<Message>,
) -> Json<String> {
    let mut msgs = messages.lock().unwrap();
    msgs.push(msg.clone());

    Json(format!("Message from {} received!", msg.name))
}

async fn get_messages(State(messages): State<MessageList>) -> Json<Vec<Message>> {
    let msgs = messages.lock().unwrap();
    Json(msgs.clone())
}

async fn root() -> Html<&'static str> {
    Html("<h1>Server Works</h1>")
}

#[tokio::main]
async fn main() {
    let messages: MessageList = Arc::new(Mutex::new(Vec::new()));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/api/messages", get(get_messages).post(handle_message))
        .with_state(messages)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
