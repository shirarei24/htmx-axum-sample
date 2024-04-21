use axum::{
    extract::{Form, Path},
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};

use lazy_static::lazy_static;
use std::sync::Mutex;

use askama::Template;
#[derive(Template)]
#[template(path = "tasks.html")]
struct TasksTemplate<'a> {
    tasks: &'a Vec<String>,
}

#[derive(Template)]
#[template(path = "todo_list.html")]
#[allow(dead_code)]
struct TodoListTemplate<'a> {
    tasks: &'a Vec<String>,
}

lazy_static! {
    static ref TASKS: Mutex<Vec<String>> = Mutex::new(vec![]);
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/assets", tower_http::services::ServeDir::new("assets"))
        .route("/", get(show_tasks))
        .route("/add", post(add_task))
        .route("/delete/:id", post(delete_task));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn show_tasks() -> Html<String> {
    let tasks = TASKS.lock().unwrap();
    let template = TodoListTemplate { tasks: &tasks };
    Html(template.render().unwrap())
}

async fn add_task(Form(input): Form<AddTask>) -> impl IntoResponse {
    let mut tasks = TASKS.lock().unwrap();
    tasks.push(input.task);
    let template = TasksTemplate { tasks: &tasks };
    Html(template.render().unwrap())
}

async fn delete_task(Path(id): Path<usize>) -> impl IntoResponse {
    println!("deleting task {}", id);
    let mut tasks = TASKS.lock().unwrap();
    if id < tasks.len() {
        tasks.remove(id);
    }
    let template = TasksTemplate { tasks: &tasks };
    Html(template.render().unwrap())
}

#[derive(serde_derive::Deserialize)]
struct AddTask {
    task: String,
}
