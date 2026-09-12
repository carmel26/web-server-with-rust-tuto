use axum::{
     Router, routing::get
};

mod vehicule;
use vehicule::{get_vehicules_hundler, vehicule_post_handler};

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

