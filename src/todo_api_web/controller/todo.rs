use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

use crate::todo_api_web::model::{TodoCard, TodoIdResponse};

pub async fn create_todo(_info: web::Json<TodoCard>) -> impl Responder {
    let new_id = Uuid::new_v4();
    HttpResponse::Created()
        .content_type("application/json")
        .body(
            serde_json::to_string(&TodoIdResponse::new(new_id))
                .expect("failed to serialize ContactsBatchResponseId"),
        )
}
