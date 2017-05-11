#[macro_use]
extern crate serde_derive;

extern crate dotenv;
extern crate iron;
extern crate router;
extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate hyper_native_tls;
extern crate job_scheduler;
extern crate regex;

use std::env;
use std::fmt::Display;
use std::thread;
use std::time::Duration;
use iron::prelude::{Iron, IronResult, Request, Response};
use iron::status;
use router::Router;
use job_scheduler::{Job, JobScheduler};

mod webhook;
mod request;

fn handler(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}

fn greeting<T: Display>(message: T) {
    let room_id = env::var("GROUP_ID").expect("GROUP_ID is missing");
    request::push(request::Push {
        to: room_id,
        messages: vec![request::Message::Text { text: format!("{}", message) }],
    });
}

static JOB_EXPRESSION: &'static str = "0 0 8 * * Mon-Fri *";

fn register_cron_job(exp: &'static str) {
    thread::spawn(move || {
        let mut schedule = JobScheduler::new();
        schedule.add(Job::new(exp.parse().unwrap(), || {
            let greeting_message = env::var("GREETING_MESSAGE").unwrap_or("".to_owned());
            greeting(greeting_message);
        }));

        loop {
            schedule.tick();
            thread::sleep(Duration::from_secs(30));
        }
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

    register_cron_job(JOB_EXPRESSION);

    match Iron::new(router).http(format!("0.0.0.0:{}", port)) {
        Ok(success) => {
            println!("{:?}", success);
        }
        Err(error) => println!("{}", error), 
    };
}
