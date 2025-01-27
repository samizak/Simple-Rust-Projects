use actix_web::{get, App, HttpResponse, HttpServer, Responder};

// Define a handler function for GET requests
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, this is a GET request response!")
}

#[get("/api")]
async fn api_endpoint() -> impl Responder {
    HttpResponse::Ok().body("API endpoint response")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Configure and start the HTTP server
    HttpServer::new(|| {
        App::new()
            .service(hello) // Register the handler
            .service(api_endpoint)
    })
    .bind("127.0.0.1:8080")? // Bind to localhost port 8080
    .run()
    .await
}
