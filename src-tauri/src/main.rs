// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::net::SocketAddr;

mod db;
mod schema;

use crate::db::models::{NewPost, Post};
use axum::routing::post;
use axum::Json;
use axum::{http::Method, response::IntoResponse, routing::get, Router};
use db::db::establish_connection;
use diesel::sql_types::Json;
use tower_http::cors::{Any, CorsLayer};

struct Port(u16);

#[tauri::command]
fn get_port(port: tauri::State<Port>) -> Result<String, String> {
    Ok(format!("{}", port.0))
}

fn main() {
    let port = portpicker::pick_unused_port().expect("failed to find unused port");
    println!("PORT: {}", port);
    tauri::async_runtime::spawn(app(port));
    tauri::Builder::default()
        .manage(Port(port))
        .invoke_handler(tauri::generate_handler![get_port])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn app(port: u16) {
    let _conn = &mut establish_connection();
    let app = Router::new()
        .route("/", get(root))
        .route("/login", post(login))
        .route("/posts", get(posts))
        .route("/createposts", 
            post(createpost))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(vec![
            Method::POST,
            Method::GET,
            Method::PATCH,
        ]));
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Backend on: http://{}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap()
}

async fn root() -> impl IntoResponse {
    "hello world this route is: /"
}

async fn login() -> impl IntoResponse {
    todo!()
}

async fn posts() -> impl IntoResponse {
    todo!()
}

async fn createpost(Json(payload): Json<NewPost<'_>>) -> impl IntoResponse {
    let post = Post {
        id: 0,
        title: payload.title.into(),
        body: payload.body.into(),
        published: true,
    };
    Json(post)
}
