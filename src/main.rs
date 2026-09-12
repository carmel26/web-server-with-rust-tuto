use axum::{
    Json, Router, debug_handler, routing::get
};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    // create the axum router
    let route01 = Router::new().route("/vehicule", get (get_vehicules_hundler).post(vehicule_post_handler));

    // 2. Define the IP and port for our listener (TCP)
    let address = "0.0.0.0:6579";
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("Failed to bind to address");

    // 3.axum serve to lunch the web server
    axum::serve(listener, route01.into_make_service())
        .await
        .unwrap() 
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Vehicule {
    manufacturer : String,
    model: String,
    year: u16,
    id: String,
}

#[debug_handler]
async fn get_vehicules_hundler() -> Json<Vehicule> {
    println!("Caller retrieved vehicule data from Axum");
    Json::from(
        Vehicule{
            manufacturer: "Toyota".to_string(),
            model: "Corolla".to_string(),
            year: 2020,
            id: Uuid::new_v4().to_string(),
        }
    )
}

async fn vehicule_post_handler() {
}