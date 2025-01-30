use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn main_path() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[get("/shorten")]
async fn shorten_url() -> impl Responder {
    HttpResponse::Ok().body("Shortening URL...")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(main_path).service(shorten_url))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
