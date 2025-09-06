mod controller;
mod model;
mod user_service;

use axum::{
    Router,
    routing::{get, post, put, delete},
    extract::Extension,
    serve,
};
use std::{net::SocketAddr, sync::Arc};
use user_service::UserService;
use tokio::net::TcpListener;
#[tokio::main]
async fn main() {
       let addr: SocketAddr = "0.0.0.0:3000".parse().unwrap();
    println!("Starting server on http://localhost:3000");

    // Inicializamos el servicio de base de datos
    let service = UserService::new()
        .await
        .expect("failed to connect to database");
    let service = Arc::new(service);

    // Router con las rutas y controladores
    let app = Router::new()
        .route("/users", get(controller::list_users))
        .route("/users/:id", get(controller::get_user_by_id))
        .route("/users", post(controller::create_user))
        .route("/users/:id", put(controller::update_user))  
        .route("/users/:id", delete(controller::delete_user))
        .layer(Extension(service));

     let listener = TcpListener::bind(addr).await.unwrap();

    // 🚀 Y se lo pasamos a serve
    serve(listener, app.into_make_service())
        .await
        .unwrap();
}
