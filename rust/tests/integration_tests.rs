use actix_web::{test, web, App};
use rust_backend_app::handlers::{get_todos, AppState};
use std::sync::Mutex;

#[actix_web::test]
async fn test_get_todos() {
    let app_state = web::Data::new(AppState {
        todo_list: Mutex::new(Vec::new()),
    });

    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .route("/todos", web::get().to(get_todos)),
    )
    .await;

    let req = test::TestRequest::get().uri("/todos").to_request();
    let resp = test::call_and_read_body(&app, req).await;

    let resp_body = String::from_utf8(resp.to_vec()).unwrap();
    assert_eq!(resp_body, "[]");
}
