use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use rust_backend_app::handlers::{get_todos, add_todo, AppState};
use std::sync::Mutex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        todo_list: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(app_state.clone())
            .wrap(cors)
            .route("/todos", web::get().to(get_todos))
            .route("/todos", web::post().to(add_todo))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
