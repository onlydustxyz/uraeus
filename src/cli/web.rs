use actix_web::{http::header::ContentType, web, App, HttpResponse, HttpServer, Responder};
use mime_guess::from_path;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "app/.svelte-kit/output/server/"]
struct Asset;

fn handle_embedded_file(path: &str) -> HttpResponse {
    match Asset::get(path) {
        Some(content) => HttpResponse::Ok()
            .content_type(from_path(path).first_or_octet_stream().as_ref())
            .body(content.data.into_owned()),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

#[actix_web::get("/")]
async fn index() -> impl Responder {
    handle_embedded_file("index.js")
}

#[actix_web::get("/api")]
async fn api() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body("{\"message\": \"Hello, world!\"}")
}

#[actix_web::get("/{_:.*}")]
async fn dist(path: web::Path<String>) -> impl Responder {
    handle_embedded_file(path.as_str())
}

#[actix_web::main]
pub async fn service() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(api).service(dist))
        .bind("127.0.0.1:7878")?
        .run()
        .await
}
