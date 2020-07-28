use actix_web::{middleware, App, HttpServer, http, };
use dotenv::dotenv;
use std::env;
use actix_cors::Cors;

mod apicall;
mod error_handler;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let port = 8445;
    HttpServer::new(move||{
        App::new()
            .wrap(
                Cors::new()
                    .allowed_origin("*")
                    .allowed_methods(vec!["GET","POST"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish()
            )
            .wrap(middleware::Logger::default())
            .configure(apicall::init_routes)
    })
        .bind(("0.0.0.0", port))?
        .run()
        .await

}

