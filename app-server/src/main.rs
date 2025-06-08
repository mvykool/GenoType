use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

// new types
mod types;
use types::Person;

#[tokio::main]

async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/people", get(get_people))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, world"
}

async fn get_people() -> impl IntoResponse {
    let people = vec![
        Person {
            name: String::from("Person A"),
            age: 36,
            favorite_food: Some(String::from("Pizza")),
        },
        Person {
            name: String::from("Person b"),
            age: 10,
            favorite_food: Some(String::from("coca-cola")),
        },
        Person {
            name: String::from("Person C"),
            age: 20,
            favorite_food: None,
        },

    ];

    (StatusCode::OK, Json(people))
}
