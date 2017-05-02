use std::io::Read;
use iron::prelude::{ IronResult, Request, Response};
use iron::status;
use serde_json;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum Message {
    #[serde(rename = "text")]
    Text { id: String, text: String },
    
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "sticker")]
    Sticker { id: String, package_id: String, sticker_id: String },
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

pub fn index_post(req: &mut Request) -> IronResult<Response> {
    let mut buf = String::new();
    req.body.read_to_string(&mut buf).unwrap();
    
    match parse_request_body(&buf) {
        Ok(request_body) => {
          println!("{:?}", request_body);
          Ok(Response::with((status::Ok, "{}")))
        },
        Err(err) => {
          println!("{:?}", err);
          Ok(Response::with((status::Ok, "{}")))
        },
    }
    
}

fn parse_request_body(s: &String) -> serde_json::Result<WebHook> {
  serde_json::from_str::<WebHook>(s)
}

mod tests {
  use super::*;

  #[test]
  fn it_should_parse_user_group() {
      let result = parse_request_body(&r#"
      {
        "events": [{
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
        }]   
      } 
      "#.to_owned());
      assert_eq!(result.unwrap(), WebHook {
        events: vec![Event {
          event_type: "message".to_owned(),
          reply_token: "reply-token".to_owned(),
          timestamp: 1493644768038,
          source: Source::Group {
            group_id: "group-id".to_owned(),
          },
          message: Message::Text {
            id: "message-id".to_owned(),
            text: "sample text".to_owned(),
          },
        }]
      });
  }

  #[test]
  fn it_should_parse_sticker_message() {
      let result = parse_request_body(&r#"
      {
        "events": [{
          "type": "message",
          "replyToken": "reply-token",
          "source": {
            "roomId": "room-id",
            "type": "room"
          },
          "timestamp": 1493644768038,
          "message": {
            "type": "sticker",
            "id": "message-id",
            "packageId": "package-id",
            "stickerId": "sticker-id"
          }
        }]   
      } 
      "#.to_owned());
      assert_eq!(result.unwrap(), WebHook {
        events: vec![Event {
          event_type: "message".to_owned(),
          reply_token: "reply-token".to_owned(),
          timestamp: 1493644768038,
          source: Source::Room {
            room_id: "room-id".to_owned(),
          },
          message: Message::Sticker {
            id: "message-id".to_owned(),
            package_id: "package-id".to_owned(),
            sticker_id: "sticker-id".to_owned(),
          },
        }]
      });
  }
}
