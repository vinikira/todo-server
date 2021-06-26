use rusoto_dynamodb::{DynamoDb, PutItemInput};
use uuid::Uuid;

use crate::todo_api::model::TodoCardDb;

use super::helpers::{client, TODO_CARD_TABLE};

pub fn put_todo(todo_card: TodoCardDb) -> Option<Uuid> {
    let client = client();

    let put_item_input = PutItemInput {
        table_name: TODO_CARD_TABLE.to_string(),
        item: todo_card.clone().into(),
        ..PutItemInput::default()
    };

    match client.put_item(put_item_input).sync() {
        Ok(_) => Some(todo_card.id),
        Err(_) => None,
    }
}
