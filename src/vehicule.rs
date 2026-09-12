use std::sync::Mutex;

use axum::{
    Json, debug_handler,
};
use uuid::Uuid;

// an array to store a list the vehicules
static VEHICULES: Mutex<Vec<Vehicule>> = Mutex::new(Vec::new());

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Vehicule {
    manufacturer : String,
    model: String,
    year: u16,
    id: Option<String>,
}

#[debug_handler]
pub async fn get_vehicules_hundler() -> Json<Vec<Vehicule>> {
    println!("Caller retrieved vehicule data from Axum");
     Json(VEHICULES.lock().unwrap().clone())
}

pub async fn vehicule_post_handler(Json(mut v): Json<Vehicule>) -> Json<Vehicule> {
    println!("Manufacturer: {}, Model: {}, Year: {}", v.manufacturer, v.model, v.year);
    v.id = Some(Uuid::new_v4().to_string());
    VEHICULES.lock().unwrap().push(v.clone());
    Json::from(v)
}