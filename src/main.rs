extern crate hyper;
extern crate futures;

use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};

struct HelloWorld;

const PHRASE: &'static str = "Hello, World!";

impl Service for HelloWorld {
    // boilerplate hooking up hyper's server types
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    // The future representing the eventual Response your call will
    // resolve to. This can change to whatever Future you need.
    type Future = futures::future::FutureResult<Self::Response, Self::Error>;

    fn call(&self, _req: Request) -> Self::Future {
        // We're currently ignoring the Request
        // And returning an 'ok' Future, which means it's ready
        // immediately, and build a Response with the 'PHRASE' body.
        // println!("Processing Request!!");
        futures::future::ok(
            Response::new()
                .with_header(ContentLength(PHRASE.len() as u64))
                .with_body(PHRASE)
        )
    }
}

fn main() {
    let a = 5;
    println!("My number: {}", a);
    let addr = "0.0.0.0:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(HelloWorld)).unwrap();
    server.run().unwrap();
}

#[test]
fn my_test() {

}