extern crate actix_web;

use actix_web::{http, server, App, HttpRequest};
use std::cell::Cell;

// This struct represents state
struct AppState {
    counter: Cell<usize>,
}

fn index(req: HttpRequest<AppState>) -> String {
    let count = req.state().counter.get() + 1;
    req.state().counter.set(count);

    format!("Request number: {}", count)
}

fn new_app() -> App<AppState> {
    App::with_state(AppState {
        counter: Cell::new(0),
    }).resource("/", |r| r.method(http::Method::GET).f(index))
}

fn main() {
    server::new(|| new_app())
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}
