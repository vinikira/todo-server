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