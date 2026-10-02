use askama::Template;
use axum::{Router, response::Html, routing::get};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json_file = include_bytes!("../data.json");
    let data: Vec<Item> = serde_json::from_slice(json_file)?;
    let route = Router::new().route(
        "/",
        get(Html(RootTmpl { data }.render().unwrap_or("ok".to_string()))),
    );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8089").await?;
    Ok(axum::serve(listener, route).await?)
}

#[derive(serde::Deserialize)]
struct Item {
    src: String,
    desc: String,
    name: String,
}

#[derive(Template)]
#[template(path = "root.html")]
struct RootTmpl {
    data: Vec<Item>,
}
