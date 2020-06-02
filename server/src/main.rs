use actix_files::Files;
use actix_web::{http, middleware, web, App};
use actix_web::{HttpResponse, HttpServer, Responder};
use code_gen::protos::game_request::GameRequest;
use env_logger::Env;
use server::domain::game;
use server::domain::model;
use server::domain::model::FormData;
use server::domain::model::Model;
use std::io;
use std::sync::Mutex;

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
async fn post_game(form: web::Form<FormData>, model: web::Data<Mutex<Model>>) -> impl Responder {
    let mut data = model.lock().unwrap();
    data.add_game(game::init(&form.game_name));
    HttpResponse::Ok().body(format!(
        "Game Name: {}, Num_Players: {}",
        form.game_name, form.num_players
    ))
}

async fn proto_test() -> impl Responder {
    let mut out_response = GameRequest::new();
    out_response.set_gameName("Response Game".to_string());
    out_response.set_numPlayers(55);
    HttpResponse::Ok().body(out_response.take_gameName())
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    use actix_cors::Cors;
    use middleware::Logger;

    env_logger::from_env(Env::default().default_filter_or("info")).init();
    let global_state = web::Data::new(Mutex::new(model::init(205_693_129)));

    HttpServer::new(move || {
        App::new()
            .app_data(global_state.clone())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(
                Cors::new()
                    .allowed_origin("http://localhost:8080")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
            .route("/games/count", web::get().to(game_count))
            .route("/game/create", web::post().to(post_game))
            .route("/game/resp", web::get().to(proto_test))
            .service(Files::new("/game", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:2943")?
    .run()
    .await
}
