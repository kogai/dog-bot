#[macro_use]
extern crate serde_derive;

extern crate dotenv;
extern crate iron;
extern crate router;
extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate hyper_native_tls;

use std::env;
use iron::prelude::{Iron, IronResult, Request, Response};
use iron::status;
use router::Router;

mod webhook;
mod request;

fn handler(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}

fn greeting(server: &hyper::server::Listening) {
    let room_id = env::var("GROUP_ID").expect("GROUP_ID is missing");
    request::push(request::Push {
        to: room_id,
        messages: vec![
            request::Message::Text { text: format!("犬botが{:?}で起動します...", server.socket) }
        ],
    });
}

fn main() {
    dotenv::dotenv().ok();

    let port = match env::var("PORT") {
        Ok(p) => p,
        Err(_) => "3000".to_string(),
    };

    let mut router = Router::new();
    router.get("/", handler, "index");
    router.post("/", webhook::index_post, "index_post");
    
    match Iron::new(router).http(format!("0.0.0.0:{}", port)) {
        Ok(success) => greeting(&success),
        Err(error) => println!("{}", error) 
    };
}
