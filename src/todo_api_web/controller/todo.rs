use actix_web::{web, HttpResponse, Responder};

use crate::todo_api::{
    db::todo::put_todo,
    model::{TodoCard, TodoCardDb, TodoIdResponse},
};

pub async fn create_todo(info: web::Json<TodoCard>) -> impl Responder {
    let todo_card = TodoCardDb::new(info);

    match put_todo(todo_card) {
        None => HttpResponse::BadRequest().body("Failed to create todo card."),
        Some(id) => HttpResponse::Created()
            .content_type("application/json")
            .body(
                serde_json::to_string(&TodoIdResponse::new(id))
                    .expect("failed to serialize ContactsBatchResponseId"),
            ),
    }
}
