use actix_files::Files;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;

use server::domain::model;
use server::domain::model::Model;
use server::domain::model::FormData;
use std::io;

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

/// POST /game/create This
/// function will be called from a post request
/// 
async fn post_game(form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "Game Name: {}, Num_Players: {}",
        form.game_name, form.num_players
    ))
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    use middleware::Logger;
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
            .data(model::init(205693129))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
            .route("/games/count", web::get().to(game_count))
            .route("/game/create", web::post().to(post_game))
            .service(Files::new("/game", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:2943")?
    .run()
    .await
}
