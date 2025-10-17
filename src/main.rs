use actix_web::{App, HttpServer, middleware};

use crate::handlers::{health_handler, server_time_handler};

mod handlers;
mod models;

static SERVER_ADDRESS: &str = "0.0.0.0";
static SERVER_PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // init logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // init app instance
    let app = || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(health_handler)
            .service(server_time_handler)
    };

    // start http server
    HttpServer::new(app)
        .bind((SERVER_ADDRESS, SERVER_PORT))?
        .run()
        .await?;

    Ok(())
}
