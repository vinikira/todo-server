use std::collections::HashMap;

use actix_web::web;
use rusoto_dynamodb::AttributeValue;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    is_done: bool,
    title: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum State {
    Todo,
    Doing,
    Done,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoCard {
    title: String,
    description: String,
    owner: Uuid,
    tasks: Vec<Task>,
    state: State,
}

#[derive(Serialize, Deserialize)]
pub struct TodoIdResponse {
    id: Uuid,
}

impl TodoIdResponse {
    pub fn get_id(self) -> String {
        format!("{}", self.id)
    }
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone)]
struct TaskDb {
    is_done: bool,
    title: String,
}

impl TaskDb {
    fn to_db_val(self) -> AttributeValue {
        let mut task_hash = HashMap::new();
        task_hash.insert("title".to_string(), val!(S => Some(self.title.clone())));
        task_hash.insert("is_done".to_string(), val!(B => self.is_done));

        val!(M => task_hash)
    }
}

#[derive(Debug, Clone)]
enum StateDb {
    Todo,
    Doing,
    Done,
}

impl std::fmt::Display for StateDb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct TodoCardDb {
    pub id: Uuid,
    title: String,
    description: String,
    owner: Uuid,
    tasks: Vec<TaskDb>,
    state: StateDb,
}

impl TodoCardDb {
    pub fn new(card: web::Json<TodoCard>) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: card.title.clone(),
            description: card.description.clone(),
            owner: card.owner,
            tasks: card
                .tasks
                .iter()
                .map(|task| TaskDb {
                    is_done: task.is_done,
                    title: task.title.clone(),
                })
                .collect::<Vec<TaskDb>>(),
            state: match card.state {
                State::Doing => StateDb::Doing,
                State::Todo => StateDb::Todo,
                State::Done => StateDb::Done,
            },
        }
    }
}

impl Into<HashMap<String, AttributeValue>> for TodoCardDb {
    fn into(self) -> HashMap<String, AttributeValue> {
        let mut todo_card = HashMap::new();

        todo_card.insert("id".to_string(), val!(S => Some(self.id.to_string())));
        todo_card.insert("title".to_string(), val!(S => Some(self.title.to_string())));
        todo_card.insert(
            "description".to_string(),
            val!(S => Some(self.description.to_string())),
        );
        todo_card.insert("owner".to_string(), val!(S => Some(self.owner.to_string())));
        todo_card.insert("state".to_string(), val!(S => Some(self.state.to_string())));
        todo_card.insert("tasks".to_string(),
                         val!(L => self.tasks.into_iter().map(|task| task.to_db_val()).collect::<Vec<AttributeValue>>()));

        todo_card
    }
}
