use actix_web::{
    get, post,
    web::{self, Json, Path, Query, Redirect},
    App, HttpResponse, HttpServer, Responder,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Row, Sqlite}; // Import the `Row` trait

#[derive(Deserialize)]
struct QueryParams {
    pub url: String,
}

#[derive(Serialize)]
struct ShortenedUrl {
    url: String,
    shortened_id: String,
    shortened_url: String,
}

// Struct to represent a single row in the `urls` table
#[derive(Serialize)]
struct UrlEntry {
    id: String,
    original_url: String,
}

// GET handler for the root path
#[get("/")]
async fn main_path() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[get("/{short_id}")]
async fn redirect(short_id: Path<String>, db: web::Data<Pool<Sqlite>>) -> impl Responder {
    let short_id = short_id.into_inner();
    println!("Extracted short_id: {:?}", short_id);

    let row = sqlx::query("SELECT original_url FROM urls WHERE id = ?")
        .bind(&short_id)
        .fetch_optional(db.get_ref())
        .await;

    match row {
        Ok(Some(row)) => {
            let original_url: String = row.get("original_url");
            // Return JSON instead of redirect
            HttpResponse::Ok().json(json!({
                "short_id": short_id,
                "original_url": original_url,
                "message": "Successfully resolved URL"
            }))
        }
        Ok(None) => HttpResponse::NotFound().json(json!({
            "error": "URL not found",
            "short_id": short_id
        })),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "error": "Database error",
                "details": format!("{:?}", e)
            }))
        }
    }
}

#[post("/shorten")]
async fn shorten_url(query: Query<QueryParams>, db: web::Data<Pool<Sqlite>>) -> impl Responder {
    let mut url = query.url.trim().to_string();

    // Add http:// scheme if missing
    if !url.starts_with("http://") && !url.starts_with("https://") {
        url = format!("http://{}", url);
    }

    let shortened_id = generate_shortened_id(6);
    let shortened_url = format!("http://localhost:8080/{}", shortened_id);

    sqlx::query("INSERT INTO urls (id, original_url) VALUES (?, ?)")
        .bind(&shortened_id)
        .bind(&url)
        .execute(db.get_ref())
        .await
        .expect("Failed to insert into database");

    HttpResponse::Ok().json(ShortenedUrl {
        url: url.to_string(),
        shortened_id,
        shortened_url,
    })
}

// GET handler to retrieve all URLs from the database
#[get("/get-urls")]
async fn get_urls(db: web::Data<Pool<Sqlite>>) -> impl Responder {
    // Fetch all rows from the `urls` table
    let rows = sqlx::query("SELECT id, original_url FROM urls")
        .fetch_all(db.get_ref())
        .await
        .expect("Failed to query database");

    // Convert rows into a vector of `UrlEntry` structs
    let urls: Vec<UrlEntry> = rows
        .into_iter()
        .map(|row| UrlEntry {
            id: row.get("id"),
            original_url: row.get("original_url"),
        })
        .collect();

    // Return the URLs as a JSON response
    HttpResponse::Ok().json(urls)
}

// Helper function to generate a random shortened ID
fn generate_shortened_id(length: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| rng.gen_range(b'a'..=b'z') as char) // Generate random lowercase letters
        .collect()
}

// Main function to start the server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Connect to the SQLite database
    let db = SqlitePoolOptions::new()
        .connect("sqlite:urls.db")
        .await
        .expect("Failed to connect to SQLite database");

    // Create the `urls` table if it doesn't exist
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS urls (
            id TEXT PRIMARY KEY,
            original_url TEXT NOT NULL
        )",
    )
    .execute(&db)
    .await
    .expect("Failed to create table");

    // Start the Actix Web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone())) // Share the database connection pool
            .service(main_path) // Root path
            .service(shorten_url) // Shorten URL endpoint
            .service(get_urls) // Get all URLs endpoint (specific route)
            .service(redirect) // Redirect endpoint (wildcard route)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
