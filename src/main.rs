extern crate dotenv;
extern crate iron;
extern crate router;

use std::env;
use iron::prelude::{Iron, IronResult, Request, Response};
use iron::status;
use router::Router;

fn handler(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}

fn main() {
    dotenv::dotenv().ok();

    let port = match env::var("PORT") {
        Ok(p) => p,
        Err(_) => "3000".to_string(),
    };

    let mut router = Router::new();
    router.get("/", handler, "index");
    
    match Iron::new(router).http(format!("localhost:{}", port)) {
        Ok(success) => println!("{:?}", success),
        Err(error) => println!("{}", error) 
    };
}
