use actix_files as fs;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};

const DIR: &'static str = "/dist";
const IDX_FILE: &'static str = "index.html";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    tracing_subscriber::fmt::init();

    let serv_dir = std::env::var("SERV_DIR").unwrap_or({
        tracing::info!("using default directory: `{}`", DIR);
        DIR.to_string()
    });

    let idx_file = std::env::var("IDX_FILE").unwrap_or({
        tracing::info!("presumed index file: `{}`", IDX_FILE);
        IDX_FILE.to_string()
    });

    HttpServer::new(move || {
        App::new().service(fs::Files::new("/", &serv_dir).index_file(&idx_file))
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
