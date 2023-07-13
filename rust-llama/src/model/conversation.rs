use serde::Deserialize;
use serde::Serialize;

pub struct Message {
    pub text: String,
    pub user: bool,
}

pub struct Conversation {
    pub messages: Vec<Message>
}