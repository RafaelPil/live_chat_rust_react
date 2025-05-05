use axum::{
    response::Html, routing::{get, post}, Json, Router
};
use http::Method;
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};

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
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST]);

    let app = Router::new()
        .route("/", get(root))
        .route("/api/messages", post(handle_message))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
