use regex;
use rocket_contrib::Json;

use conversation;
use request;
use request::{reply, Reply};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum Message {
    #[serde(rename = "text")]
    Text { id: String, text: String },

    #[serde(rename_all = "camelCase")]
    #[serde(rename = "sticker")]
    Sticker {
        id: String,
        package_id: String,
        sticker_id: String,
    },
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum Source {
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "group")]
    Group { group_id: String },

    #[serde(rename_all = "camelCase")]
    #[serde(rename = "room")]
    Room { room_id: String },

    #[serde(rename_all = "camelCase")]
    #[serde(rename = "user")]
    User { user_id: String },
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    #[serde(rename = "type")]
    event_type: String,
    reply_token: String,
    timestamp: isize,
    source: Source,
    message: Message,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct WebHook {
    events: Vec<Event>,
}

#[post("/", format = "application/json", data = "<payload>")]
pub fn index_post(payload: Json<WebHook>) {
    let conversations = &conversation::CONVERSATION.initial_conversations;

    for event in &payload.events {
        match &event.message {
            &Message::Text { ref text, .. } => {
                for &(ref regex_str, ref response) in conversations {
                    let reg = regex::Regex::new(regex_str).unwrap();
                    if reg.is_match(text) {
                        reply(Reply {
                            reply_token: event.reply_token.clone(),
                            messages: vec![request::Message::Text {
                                text: response.to_string(),
                            }],
                        });
                    }
                }
            }
            _ => {}
        }
    }
}
