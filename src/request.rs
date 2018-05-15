use std::env;

use hyper::client::Response;
use hyper::header::{Authorization, ContentType, Headers};
use reqwest::{self, Client};
use serde;
use serde_json;

const LINE_API: &'static str = "https://api.line.me/v2/bot/message";
const PUSH: &'static str = "/push";
const REPLY: &'static str = "/reply";

fn header() -> Headers {
    let mut headers = Headers::new();
    headers.set(ContentType::json());

    let token = env::var("CHANNEL_ACCESS_TOKEN").expect("CHANNEL_ACCESS_TOKEN is missing");
    headers.set(Authorization(format!("Bearer {}", token)));

    headers
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum Message {
    #[serde(rename = "text")]
    Text { text: String },
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Push {
    pub to: String,
    pub messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Reply {
    pub reply_token: String,
    pub messages: Vec<Message>,
}

fn post<T: serde::Serialize>(path: &str, payload: &T) -> reqwest::Result<Response> {
    let client = Client::new();
    let header = header();
    let url = format!("{}{}", LINE_API, path);
    let res = client
        .post(url.as_str())
        .headers(header)
        .body("the exact body that is sent")
        .send()?;

    unimplemented!();
}

pub fn push(payload: Push) {
    match post(PUSH, &payload) {
        Ok(success) => println!("Status: {:?}", success.status()),
        Err(error) => println!("{:?}", error),
    };
}

pub fn reply(payload: Reply) {
    match post(REPLY, &payload) {
        Ok(success) => println!("Status: {:?}", success.status()),
        Err(error) => println!("{:?}", error),
    };
}
