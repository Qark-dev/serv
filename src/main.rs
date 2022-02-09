use actix_files as fs;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};

const DIR: &'static str = "/dist";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    tracing_subscriber::fmt::init();

    let serv_dir = std::env::var("SERV_DIR").unwrap_or({
        tracing::info!("using default directory: `{}`", DIR);
        DIR.to_string()
    });

    HttpServer::new(move || {
        App::new().service(fs::Files::new("/", &serv_dir).index_file("index.html"))
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
