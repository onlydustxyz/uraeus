// use crate::cli::compare;
use actix_cors::Cors;
use actix_web::{http::header::ContentType, web, App, HttpResponse, HttpServer, Responder};
use mime_guess::from_path;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};

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

#[derive(Deserialize)]
struct VerifyInput {
    address: String,
}

#[derive(Serialize)]
struct VerifyOutput {
    success: String,
}

#[derive(Serialize)]
struct SourcesOutput {
    sources: Vec<String>,
}

#[actix_web::post("/api/verify")]
async fn verify(query: web::Json<VerifyInput>) -> impl Responder {
    let response = &mut VerifyOutput {
        success: "false".to_string(),
    };

    if query.address == "0x0000000000000000000000000000000000000001" {
        response.success = "true".to_string();
    }
    let serialized = serde_json::to_string(&response).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serialized)
}

#[actix_web::get("/api/sources")]
async fn sources() -> impl Responder {
    let sources = vec!["main".to_string()];
    let response = &SourcesOutput { sources };
    let serialized = serde_json::to_string(&response).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serialized)
}

#[actix_web::get("/{_:.*}")]
async fn dist(path: web::Path<String>) -> impl Responder {
    handle_embedded_file(path.as_str())
}

#[actix_web::main]
pub async fn service() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // `permissive` is a wide-open development config
            .wrap(Cors::permissive())
            .service(index)
            .service(verify)
            .service(sources)
            .service(dist)
    })
    .bind("127.0.0.1:7878")?
    .run()
    .await
}
