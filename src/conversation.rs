use serde_json;
use std::fs;
use std::io;

#[derive(Serialize, Deserialize)]
pub struct Conversation {
  pub greeting: String,
  pub initial_conversations: Vec<(String, String)>,
}

#[derive(Debug)]
pub enum Error {
  FileNotfound(io::Error),
  ParseFailed(serde_json::Error),
}

impl Conversation {
  pub fn new() -> Result<Self, Error> {
    fs::read(".conversation.json")
      .map_err(|err| Error::FileNotfound(err))
      .and_then(|byte| {
        serde_json::from_slice::<Conversation>(&byte).map_err(|err| Error::ParseFailed(err))
      })
  }
}

lazy_static! {
  pub static ref CONVERSATION: Conversation = match Conversation::new() {
    Ok(conv) => conv,
    Err(err) => unreachable!("{:?}", err),
  };
}
