use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub text: String,
    pub user: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Conversation {
    pub messages: Vec<Message>
}

impl Conversation {
    pub fn new() -> Conversation {
        Conversation { 
            messages: Vec::new() 
        }
    }
}