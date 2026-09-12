use axum:: {
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let route01 = Router::new().route("/vehicule", get (get_vehicules_hundler));

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


async fn get_vehicules_hundler() {

}