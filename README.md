# Rust Axum REST API — Modules and Vehicle Handlers

A simple **Rust REST API using Axum** that demonstrates how to organize application logic into separate modules and call those modules from `main.rs`.

This tutorial is part of my journey learning **Rust web development with Axum**, with a focus on understanding Rust modules, handlers, JSON serialization, shared state, and HTTP routes.

## 📌 What This Tutorial Covers

In this project, we build a simple vehicle API that can:

* Create a vehicle using a `POST` request
* Retrieve all vehicles using a `GET` request
* Generate a unique UUID for each vehicle
* Store vehicles temporarily in memory
* Serialize and deserialize JSON using Serde
* Organize handlers in a separate Rust module
* Import and use the module from `main.rs`
* Define HTTP routes using Axum

## 🛠️ Technologies Used

* **Rust**
* **Axum** — Web framework
* **Serde** — JSON serialization/deserialization
* **UUID** — Unique vehicle identifiers
* **Tokio** — Asynchronous runtime

## 📂 Project Structure

The project is organized approximately as follows:

```text
rust-axum-vehicle-api/
│
├── src/
│   ├── main.rs
│   └── vehicules.rs
│
├── Cargo.toml
└── README.md
```

The important idea in this tutorial is separating the vehicle-related functionality from the application entry point.

### `main.rs`

The `main.rs` file is responsible for:

* Starting the Tokio runtime
* Creating the Axum router
* Defining API routes
* Connecting the routes to the handlers defined in the vehicle module
* Starting the HTTP server

### `vehicules.rs`

The `vehicules.rs` module contains:

* The `Vehicule` struct
* The in-memory vehicle collection
* The GET handler
* The POST handler

This separation makes the application easier to understand and maintain as it grows.

---

# 🚀 Getting Started

## 1. Clone the repository

```bash
git clone <YOUR_REPOSITORY_URL>
cd rust-axum-vehicle-api
```

## 2. Build the project

```bash
cargo build
```

## 3. Run the application

```bash
cargo run
```

The API will start on the address configured in `main.rs`, for example:

```text
http://127.0.0.1:3000
```

---

# 📦 Dependencies

The project uses the following dependencies in `Cargo.toml`:

```toml
[dependencies]
axum = { version = "0.8", features = ["macros"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
uuid = { version = "1", features = ["v4"] }
```

Your exact versions may differ depending on when you build the project.

---

# 🧩 Creating a Rust Module

Instead of putting everything inside `main.rs`, we create a separate module:

```text
src/
├── main.rs
└── vehicules.rs
```

The module can then be declared in `main.rs`:

```rust
mod vehicules;
```

This tells Rust that the application has another module named `vehicules`.

We can then import the handlers:

```rust
use vehicules::{
    get_vehicules_handler,
    vehicule_post_handler,
};
```

This allows `main.rs` to use functions defined inside `vehicules.rs`.

---

# 🚗 Vehicle Model

The vehicle structure is defined in `vehicules.rs`:

```rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Vehicule {
    manufacturer: String,
    model: String,
    year: u16,
    id: Option<String>,
}
```

The important Rust concepts demonstrated here are:

### `pub`

```rust
pub struct Vehicule
```

Makes the structure accessible from other modules.

### `Debug`

```rust
#[derive(Debug)]
```

Allows the structure to be printed using debugging formatting.

### `Clone`

```rust
#[derive(Clone)]
```

Allows instances of `Vehicule` to be cloned.

This is useful because the application stores vehicles in a `Vec<Vehicule>` and sometimes needs to clone the vector or individual vehicles.

### `Serialize`

```rust
serde::Serialize
```

Allows Rust structures to be converted into JSON.

### `Deserialize`

```rust
serde::Deserialize
```

Allows incoming JSON data to be converted into Rust structures.

---

# 💾 In-Memory Storage

For this tutorial, vehicles are stored in a simple vector protected by a `Mutex`:

```rust
static VEHICULES: Mutex<Vec<Vehicule>> = Mutex::new(Vec::new());
```

Conceptually:

```text
VEHICULES
    │
    ▼
  Mutex
    │
    ▼
 Vec<Vehicule>
    │
    ├── Vehicle 1
    ├── Vehicle 2
    └── Vehicle 3
```

This is useful for learning, but it is **not a replacement for a production database**.

The data will be lost when the application stops.

---

# 🔍 GET Handler

The GET handler retrieves all stored vehicles:

```rust
pub async fn get_vehicules_handler() -> Json<Vec<Vehicule>> {
    println!("Caller retrieved vehicule data from Axum");

    Json(VEHICULES.lock().unwrap().clone())
}
```

The important part is:

```rust
Json<Vec<Vehicule>>
```

because the endpoint returns a list of vehicles rather than a single vehicle.

---

# ➕ POST Handler

The POST handler receives a vehicle from the client:

```rust
pub async fn vehicule_post_handler(
    Json(mut v): Json<Vehicule>,
) -> Json<Vehicule> {
```

A UUID is then generated:

```rust
v.id = Some(Uuid::new_v4().to_string());
```

The vehicle is stored:

```rust
VEHICULES.lock().unwrap().push(v.clone());
```

Finally, the created vehicle is returned:

```rust
Json(v)
```

---

# 🛣️ Axum Routes

In `main.rs`, the handlers can be connected to routes:

```rust
let app = Router::new()
    .route(
        "/vehicules",
        get(get_vehicules_handler)
            .post(vehicule_post_handler),
    );
```

This gives us:

| HTTP Method | Endpoint     | Purpose               |
| ----------- | ------------ | --------------------- |
| `GET`       | `/vehicules` | Retrieve all vehicles |
| `POST`      | `/vehicules` | Create a new vehicle  |

---

# 🧪 Testing the API

## GET Vehicles

Using `curl`:

```bash
curl http://127.0.0.1:6579/vehicules
```

Initially, the response should be an empty array:

```json
[]
```

---

## POST a Vehicle

Send a JSON request:

```bash
curl -X POST http://127.0.0.1:6579/vehicules \
  -H "Content-Type: application/json" \
  -d '{
    "manufacturer": "Toyota",
    "model": "Corolla",
    "year": 2022,
    "id": null
  }'
```

The API generates the ID automatically.

A response will look similar to:

```json
{
  "manufacturer": "Toyota",
  "model": "Corolla",
  "year": 2022,
  "id": "generated-uuid"
}
```

---

## GET the Vehicles Again

```bash
curl http://127.0.0.1:6579/vehicules
```

You should now receive something similar to:

```json
[
  {
    "manufacturer": "Toyota",
    "model": "Corolla",
    "year": 2022,
    "id": "generated-uuid"
  }
]
```

---

# 🧠 Rust Concepts Learned

This small project demonstrates several important Rust concepts:

### 1. Modules

```rust
mod vehicules;
```

Modules allow us to organize Rust applications into multiple files.

### 2. Visibility

```rust
pub
```

The `pub` keyword allows items to be accessed outside their module.

### 3. Ownership and Borrowing

Rust's ownership system controls how data is moved, borrowed, and cloned.

### 4. `Clone`

```rust
v.clone()
```

Creates another owned copy of a value.

### 5. `Mutex`

```rust
Mutex<Vec<Vehicule>>
```

Provides synchronized access to shared mutable data.

### 6. Async Functions

Axum handlers are asynchronous:

```rust
pub async fn ...
```

### 7. JSON

Serde makes it easy to convert between Rust structures and JSON.

### 8. HTTP Routing

Axum connects HTTP methods and paths to Rust handler functions.

---

# ⚠️ Important Note About the Storage

This tutorial intentionally uses:

```rust
static VEHICULES: Mutex<Vec<Vehicule>>
```

as a simple in-memory data store.

This is suitable for learning but has several limitations:

* Data disappears when the application stops.
* It is not suitable for large datasets.
* It is not appropriate for a production application.
* It does not provide persistent storage.
* It does not provide database transactions.

A future version of the project can replace the in-memory vector with a database such as **PostgreSQL, MySQL, SQLite, or SurrealDB**.

---

# 🎯 Learning Objective

The main objective of this tutorial is not to build a production-ready vehicle management system.

Instead, it demonstrates how to move from a simple Rust program toward a more structured web application by:

```text
main.rs
   │
   ├── Define application
   │
   ├── Configure routes
   │
   └── Call handlers
          │
          ▼
    vehicules.rs
          │
          ├── Vehicle model
          ├── GET handler
          ├── POST handler
          └── In-memory storage
```

This modular approach becomes increasingly important as an Axum application grows.

---

# 📚 Next Steps

Possible improvements to this project include:

* Add `PUT` to update vehicles
* Add `DELETE` to remove vehicles
* Add validation
* Introduce proper application state
* Replace `Mutex<Vec<Vehicule>>` with a database
* Add error handling
* Add authentication and authorization
* Add automated tests
* Add API documentation with OpenAPI/Swagger
* Containerize the application with Docker

---

# 👨‍💻 Author

**Carmel**

This repository is part of my practical learning journey in **Rust, Axum, backend development, and modern systems programming**.
