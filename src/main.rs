use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn ping() -> impl Responder {
    HttpResponse::Ok().body("{\"value\":\"pong\"}")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("hello");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/ping", web::get().to(ping))
    })
    // .bind("127.0.0.1:8088")?
    .bind("0.0.0.0:8088")?
    .run()
    .await
}
