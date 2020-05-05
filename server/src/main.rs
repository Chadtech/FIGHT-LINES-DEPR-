use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

use server::domain::model;
use server::domain::model::Model;

/// Responder Objects
/// GET /
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

/// GET /again
async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello World Again")
}

/// GET /games/count, we also pass in the state
async fn game_count(model: web::Data<Model>) -> impl Responder {
    HttpResponse::Ok().body(model.games_count().to_string())
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(model::init(205693129))
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
            .route("/games/count", web::get().to(game_count))
    })
    .bind("127.0.0.1:2943")?
    .run()
    .await
}
