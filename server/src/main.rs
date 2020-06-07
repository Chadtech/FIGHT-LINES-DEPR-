use actix_files::Files;
use actix_web::HttpServer;
use actix_web::{http, middleware, web, App};
use std::io;
use std::sync::Mutex;

use server::domain::model;
use server::route;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    use actix_cors::Cors;
    use env_logger::Env;
    use middleware::Logger;

    env_logger::from_env(Env::default().default_filter_or("info")).init();
    let global_state = web::Data::new(Mutex::new(model::init(205_693_129)));

    HttpServer::new(move || {
        App::new()
            .app_data(global_state.clone())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            // .wrap(
            //     Cors::new()
            //         .allowed_origin("http://localhost:8000")
            //         .allowed_methods(vec!["GET", "POST"])
            //         .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            //         .allowed_header(http::header::CONTENT_TYPE)
            //         .finish(),
            // )
            .route("/again", web::get().to(route::index2))
            .route("/games/count", web::get().to(route::game_count))
            .route("/game/create", web::post().to(route::post_game))
            .route("/game/resp", web::get().to(route::proto_test))
            .service(Files::new("/game", "./static").index_file("index.html"))
            .service(Files::new("/assets", "../ui/assets").show_files_listing())
            .service(Files::new("/pkg", "../ui/pkg").show_files_listing())
            .default_service(web::get().to(route::index))
    })
    .bind("127.0.0.1:2943")?
    .run()
    .await
}
