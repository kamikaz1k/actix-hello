// extern crate env_logger;
use actix_web::{web, middleware, App, HttpResponse, HttpServer, Responder};

async fn ping() -> impl Responder {
    HttpResponse::Ok().body("{\"value\":\"pong\"}")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/ping", web::get().to(ping))
    })
    .bind("0.0.0.0:8088")?
    .run()
    .await
}
