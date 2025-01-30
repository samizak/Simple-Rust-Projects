use actix_web::{
    get, post,
    web::{self, Path, Query},
    App, HttpResponse, HttpServer, Responder,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Row, Sqlite};

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

#[derive(Serialize)]
struct UrlEntry {
    id: String,
    original_url: String,
}

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

#[get("/get-urls")]
async fn get_urls(db: web::Data<Pool<Sqlite>>) -> impl Responder {
    let rows = sqlx::query("SELECT id, original_url FROM urls")
        .fetch_all(db.get_ref())
        .await
        .expect("Failed to query database");

    let urls: Vec<UrlEntry> = rows
        .into_iter()
        .map(|row| UrlEntry {
            id: row.get("id"),
            original_url: row.get("original_url"),
        })
        .collect();

    HttpResponse::Ok().json(urls)
}

fn generate_shortened_id(length: usize) -> String {
    let mut rng = rand::rng();
    (0..length)
        .map(|_| rng.random_range(b'a'..=b'z') as char) // Generate random lowercase letters
        .collect()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = SqlitePoolOptions::new()
        .connect("sqlite:urls.db")
        .await
        .expect("Failed to connect to SQLite database");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS urls (
            id TEXT PRIMARY KEY,
            original_url TEXT NOT NULL
        )",
    )
    .execute(&db)
    .await
    .expect("Failed to create table");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(main_path)
            .service(shorten_url)
            .service(get_urls)
            .service(redirect)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
