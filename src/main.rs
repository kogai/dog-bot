#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate regex;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate tokio_core;

use futures::future::Future;
use rocket::config::{Config, Environment};
use rocket::custom;
use std::env;
use std::fmt::Display;
use std::thread;
use std::time::Duration;

use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};

mod conversation;
mod request;
// mod webhook;

fn greeting<T: Display>(message: T) {
    let room_id = env::var("GROUP_ID").expect("GROUP_ID is missing");
    request::push(request::Push {
        to: room_id,
        messages: vec![request::Message::Text {
            text: format!("{}", message),
        }],
    });
}

#[get("/hello")]
fn hello() -> &'static str {
    "Hello, world!"
}

fn main() {
    let port = env::var("PORT").unwrap_or("3000".to_owned());
    let port = u16::from_str_radix(&port, 10).unwrap();

    let config = Config::build(Environment::Staging)
        .address("0.0.0.0")
        .port(port)
        .finalize()
        .unwrap();

    let server = custom(config, true);
    server
        .mount("/", routes![hello])
        .attach(rocket::fairing::AdHoc::on_launch(|rocket| {
            println!("Rocket launch config: {:?}", rocket.config());
            greeting("Good morning, sir.");
        }))
        .launch();

    // let mut router = Router::new();
    // router.post("/", webhook::index_post, "index_post");
}
