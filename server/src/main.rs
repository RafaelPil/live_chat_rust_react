use axum::{
    response::Html, routing::{get, post}, Json, Router
};
use serde::{Deserialize, Serialize};
use tracing_subscriber::fmt::format;

#[derive(Deserialize)]
struct MessageInput {
    name: String,
    text: String,
}

#[derive(Serialize)]
struct MessageOutput {
    message: String,
}

async fn handle_message(Json(input): Json<MessageInput>) -> Json<MessageOutput> {
    let msg_output = MessageOutput {
        message: format!("{} from: {}", input.name, input.text),
    };

    return Json(msg_output);
}

async fn root() -> Html<&'static str> {
    Html("<h1>Server Works</h1>")
}

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(root))
    .route("/api/messages", post(handle_message));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
