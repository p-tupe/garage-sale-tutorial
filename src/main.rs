use std::fs;

use askama::Template;
use axum::{Router, routing::get};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data: Vec<Item> = serde_json::from_slice(&fs::read("data.json")?)?;
    // let data = data.to_string();

    let route = Router::new().route(
        "/",
        get(RootTmpl { data }.render().unwrap_or("ok".to_string())),
    );
    let listener = TcpListener::bind("localhost:8080").await?;

    Ok(axum::serve(listener, route).await?)
}

#[derive(serde::Deserialize)]
struct Item {
    src: String,
    desc: String,
}

#[derive(Template)]
#[template(path = "root.html")]
struct RootTmpl {
    data: Vec<Item>,
}
