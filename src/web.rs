use crate::cli::model::Config;
use crate::cli::verify;

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

// matches the source files
#[derive(RustEmbed)]
#[folder = "app/build/"]
struct Asset;

// AppConfig is a configuration used to start the web ui.
struct AppConfig {
    project_dir: String,
    build_dir: String,
}

// VerifyInput provides the input structure for the verify api.
#[derive(Deserialize)]
struct VerifyInput {
    address: String,
    source: String,
}

// VerifyInput provides the output structure for the verify api.
#[derive(Serialize)]
struct VerifyOutput {
    success: bool,
    address: String,
    source: String,
}

// VerifyInput provides the output structure for the sources api.
#[derive(Serialize)]
struct SourcesOutput {
    sources: Vec<String>,
}

// verify_api is an api endpoint that checks the deployed contract and the
// source code
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

// sources is an api endpoint that lists all the sources in the `src` folder
// of the project directory.
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

// handle_embedded_file returns a file given a path
fn handle_embedded_file(path: &str) -> HttpResponse {
    match Asset::get(path) {
        Some(content) => HttpResponse::Ok()
            .content_type(from_path(path).first_or_octet_stream().as_ref())
            .body(content.data.into_owned()),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

// index returns the route for the index page
#[actix_web::get("/")]
async fn index() -> impl Responder {
    handle_embedded_file("index.html")
}

// dist matches the path and returned the content of webapp
#[actix_web::get("/{_:.*}")]
async fn dist(path: web::Path<String>) -> impl Responder {
    handle_embedded_file(path.as_str())
}

// service starts the listener with the various services and data from the
// configuration.
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
            .app_data(Data::clone(&data))
            .service(index)
            .service(verify_api)
            .service(sources)
            .service(dist)
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await;
}
