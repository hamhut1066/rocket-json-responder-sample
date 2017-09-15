#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use rocket_contrib::Json;
use rocket::response::{Responder, Response};
use rocket::request::Request;
use rocket::http::Status;

#[derive(Serialize, Debug)]
pub struct EmptyResponse {}

#[derive(Serialize, Debug)]
pub struct MyResponder<R>(pub R);


impl<'r, R: Responder<'r>> Responder<'r> for MyResponder<R> {
    fn respond_to(self, req: &Request) -> Result<Response<'r>, Status> {
        let header = rocket::http::Header::new("Custom Header", "value");
        Response::build()
            .header(header)
            .merge(self.0.respond_to(req)?)
            .ok()
    }
}

type ResponseWrapper<A> = MyResponder<Json<A>>;
type ResponseWrapper2<A> = Json<MyResponder<A>>;

// works fine
#[get("/test")]
fn test() -> ResponseWrapper<EmptyResponse> {
    MyResponder(Json(EmptyResponse {}))
}

// now my header is being dropped...
#[get("/test2")]
fn test2() -> ResponseWrapper2<EmptyResponse> {
    Json(MyResponder(EmptyResponse {}))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![test, test2])
        .launch();
}
