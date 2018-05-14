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
extern crate serde;
extern crate serde_json;

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
// mod request;
// mod webhook;

// fn handler(req: &mut Request) -> IronResult<Response> {
//     let ref query = req.extensions
//         .get::<Router>()
//         .unwrap()
//         .find("query")
//         .unwrap_or("/");
//     Ok(Response::with((status::Ok, *query)))
// }

// fn greeting<T: Display>(message: T) {
//     let room_id = env::var("GROUP_ID").expect("GROUP_ID is missing");
//     request::push(request::Push {
//         to: room_id,
//         messages: vec![request::Message::Text {
//             text: format!("{}", message),
//         }],
//     });
// }
// struct HelloWorld;

#[get("/hello")]
fn hello() -> &'static str {
    "Hello, world!"
}

// const PHRASE: &'static str = "Hello, World!";

// impl Service for HelloWorld {
//     // boilerplate hooking up hyper's server types
//     type Request = Request;
//     type Response = Response;
//     type Error = hyper::Error;
//     // The future representing the eventual Response your call will
//     // resolve to. This can change to whatever Future you need.
//     type Future = Box<Self::Future<Item = Self::Response, Error = Self::Error>>;

//     fn call(&self, _req: Request) -> Self::Future {
//         // We're currently ignoring the Request
//         // And returning an 'ok' Future, which means it's ready
//         // immediately, and build a Response with the 'PHRASE' body.
//         Box::new(futures::future::ok(
//             Response::new()
//                 .with_header(ContentLength(PHRASE.len() as u64))
//                 .with_body(PHRASE),
//         ))
//     }
// }

fn main() {
    let port = env::var("PORT").unwrap_or("3000".to_owned());
    let port = u16::from_str_radix(&port, 10).unwrap();

    let config = Config::build(Environment::Staging)
        .address("0.0.0.0")
        .port(port)
        .finalize()
        .unwrap();

    let server = custom(config, false);
    server.mount("/", routes![hello]).launch();

    // let addr = "127.0.0.1:3000".parse().unwrap();
    // let server = Http::new().bind(&addr, || Ok(HelloWorld)).unwrap();
    // server.run().unwrap();
    // let mut router = Router::new();
    // router.get("/", handler, "index");
    // router.post("/", webhook::index_post, "index_post");

    // match Iron::new(router).http(host) {
    //     Ok(success) => {
    //         println!("{:?}", success);
    //         greeting("いぬ起きた");
    //     }
    //     Err(error) => println!("{}", error),
    // };
}
