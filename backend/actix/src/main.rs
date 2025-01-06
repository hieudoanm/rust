use actix_web::{get, App, HttpServer, Responder};
use uuid::Uuid;

#[get("/")]
async fn index() -> impl Responder {
    return "Hello ".to_owned() + &Uuid::new_v4().to_string();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
