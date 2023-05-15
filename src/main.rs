use actix_web::{App, HttpServer, HttpResponse, Responder};

async fn ping() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/ping", actix_web::web::get().to(ping))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}