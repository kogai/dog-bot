use std::io::Read;
use iron::prelude::{ IronResult, Request, Response};
use iron::status;
use serde_json;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MessageText {
  #[serde(rename = "type")]
  message_type: String,
  id: String,
  text: String,
}

#[derive(Debug)]
enum Messages {
    Text(MessageText),
    // Sticky,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SourceGroup {
  #[serde(rename = "type")]
  source_type: String,
  group_id: String,
}

#[derive(Debug)]
pub enum Source {
  Group(SourceGroup),
  Room,
  User,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Event {
  #[serde(rename = "type")]
  event_type: String,
  reply_token: String,
  timestamp: isize,
  source: SourceGroup,
  message: MessageText,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct WebHook {
  events: Vec<Event>,
}

pub fn index_post(req: &mut Request) -> IronResult<Response> {
    let mut buf = String::new();
    req.body.read_to_string(&mut buf).unwrap();
    
    let request_body = serde_json::from_str::<WebHook>(&buf);
    println!("request body: {:?}", request_body);

    Ok(Response::with((status::Ok, "{}")))
}

fn parse_request_body(s: &String) -> Event {
  serde_json::from_str::<Event>(s).unwrap()
}

mod tests {
  use super::*;

  #[test]
  fn it_should_parse_body() {
      let result = parse_request_body(&r#"
        {
            "type": "message",
            "replyToken": "reply-token",
            "source": {
              "groupId": "group-id",
              "type": "group"
            },
            "timestamp": 1493644768038,
            "message": {
              "type": "text",
              "id": "message-id",
              "text": "sample text"
            }
          }
      "#.to_owned());
      assert_eq!(result, Event {
        event_type: "message".to_owned(),
        reply_token: "reply-token".to_owned(),
        timestamp: 1493644768038,
        source: SourceGroup {
          source_type: "group".to_owned(),
          group_id: "group-id".to_owned(),
        },
        message: MessageText {
          message_type: "text".to_owned(),
          id: "message-id".to_owned(),
          text: "sample text".to_owned(),
        },
      });
  }
}
