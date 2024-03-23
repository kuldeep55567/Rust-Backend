use actix_web::{web, App, HttpServer, Responder};

async fn indexs() -> impl Responder {
    "Hello, world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start Actix-web server
    HttpServer::new(|| {
        // Create App and configure routes
        App::new().route("/", web::get().to(indexs))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}