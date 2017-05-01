use std::io::Read;
use iron::prelude::{ IronResult, Request, Response};
use iron::status;
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceGroup {
  #[serde(rename = "type")]
  source_type: String,
  group_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
  #[serde(rename = "type")]
  event_type: String,
  reply_token: String,
  timestamp: i32,
  source: SourceGroup,
}

#[derive(Debug, Serialize, Deserialize)]
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