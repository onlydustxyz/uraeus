use crate::cli::model::Config;
use crate::cli::verify;
use actix_cors::Cors;
use actix_web::{
    http::header::ContentType,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use glob::glob;
use mime_guess::from_path;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(RustEmbed)]
#[folder = "app/build/"]
struct Asset;

struct AppConfig {
    project_dir: String,
    build_dir: String,
}

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
    handle_embedded_file("200.html")
}

#[actix_web::get("/200")]
async fn fallback() -> impl Responder {
    handle_embedded_file("200.html")
}

#[derive(Deserialize)]
struct VerifyInput {
    address: String,
    source: String,
}

#[derive(Serialize)]
struct VerifyOutput {
    success: bool,
    address: String,
    source: String,
}

#[derive(Serialize)]
struct SourcesOutput {
    sources: Vec<String>,
}

#[actix_web::post("/api/verify")]
async fn verify_api(query: web::Json<VerifyInput>, data: Data<Mutex<AppConfig>>) -> impl Responder {
    let local_data = data.lock().unwrap();

    let response = &mut VerifyOutput {
        success: false,
        address: query.address.clone(),
        source: query.source.clone(),
    };

    let verify_input = &Config {
        project_dir: local_data.project_dir.clone(),
        contract_address: query.address.clone(),
        contract_name: query.source.clone(),
        build_dir: local_data.build_dir.clone(),
    };
    let mut out = false;
    match verify::run_once(verify_input) {
        Ok(v) => {
            out = v;
        }
        Err(e) => {
            println!("{}", e);
        }
    }

    if out {
        response.success = true;
    }
    let serialized = serde_json::to_string(&response).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serialized)
}

#[actix_web::get("/api/sources")]
async fn sources(data: Data<Mutex<AppConfig>>) -> impl Responder {
    let local_data = data.lock().unwrap();
    let mut sources: Vec<String> = vec![];
    let pattern = format!("{}/src/*.cairo", &local_data.project_dir);
    let files = glob(&pattern).unwrap();
    for f in files {
        let d = f.unwrap();
        let s = d.to_str().unwrap().to_string();
        let suffix = s.split('/').last().unwrap().to_string();
        let name = suffix.split('.').next().unwrap().to_string();
        sources.push(name);
    }
    let serialized = serde_json::to_string(&SourcesOutput { sources }).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serialized)
}

#[actix_web::get("/{_:.*}")]
async fn dist(path: web::Path<String>) -> impl Responder {
    handle_embedded_file(path.as_str())
}

#[actix_web::main]
pub async fn service(port: i64, project_dir: String, build_dir: String) -> std::io::Result<()> {
    return HttpServer::new(move || {
        let location = project_dir.clone();
        let build = build_dir.clone();
        let data = Data::new(Mutex::new(AppConfig {
            project_dir: location,
            build_dir: build,
        }));
        App::new()
            // `permissive` is a wide-open development config
            .app_data(Data::clone(&data))
            .wrap(Cors::permissive())
            .service(index)
            .service(fallback)
            .service(verify_api)
            .service(sources)
            .service(dist)
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await;
}
