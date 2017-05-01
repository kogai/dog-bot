extern crate dotenv;
extern crate iron;
extern crate router;

use std::env;
use std::io::Read;
use iron::prelude::{Iron, IronResult, Request, Response};
use iron::status;
use router::Router;

fn handler(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}

fn index_post(req: &mut Request) -> IronResult<Response> {
    let mut buf = String::new();
    req.body.read_to_string(&mut buf).unwrap();
    println!("request body: {}", buf);
    
    Ok(Response::with((status::Ok, "{}")))
}

fn main() {
    dotenv::dotenv().ok();

    let port = match env::var("PORT") {
        Ok(p) => p,
        Err(_) => "3000".to_string(),
    };

    let mut router = Router::new();
    router.get("/", handler, "index");
    router.post("/", index_post, "index_post");
    
    match Iron::new(router).http(format!("localhost:{}", port)) {
        Ok(success) => println!("{:?}", success),
        Err(error) => println!("{}", error) 
    };
}
