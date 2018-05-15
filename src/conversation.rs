use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Conversation {
  greeting: String,
  initial_conversations: Vec<(String, String)>,
}

impl Conversation {
  pub fn new() -> Self {
    match fs::read("./conversation.json") {
      Ok(file) => {
        unimplemented!();
      }
      Err(err) => {
        unimplemented!();
      }
    }
  }
}
