pub mod handlers {
    use actix_web::{web, HttpResponse, Responder};
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;
    use std::sync::Mutex;
    use chrono::{DateTime, Utc};

    #[derive(Serialize, Deserialize, Clone)]
    pub struct TodoItem {
        pub id: Uuid,
        pub title: String,
        pub completed: bool,
        pub created_at: DateTime<Utc>,
    }

    #[derive(Deserialize)]
    pub struct CreateTodoItem {
        pub title: String,
        pub completed: bool,
    }

    #[derive(Deserialize)]
    pub struct UpdateTodoItem {
        pub title: Option<String>,
        pub completed: Option<bool>,
    }

    pub struct AppState {
        pub todo_list: Mutex<Vec<TodoItem>>,
    }

    pub async fn get_todos(data: web::Data<AppState>) -> impl Responder {
        let todos = data.todo_list.lock().unwrap();
        HttpResponse::Ok().json(&*todos)
    }

    pub async fn add_todo(
        item: web::Json<CreateTodoItem>,
        data: web::Data<AppState>,
    ) -> impl Responder {
        let mut todos = data.todo_list.lock().unwrap();
        let new_todo = TodoItem {
            id: Uuid::new_v4(),
            title: item.title.clone(),
            completed: item.completed,
            created_at: Utc::now(),
        };
        todos.push(new_todo);
        HttpResponse::Ok().json(&*todos)
    }
}
