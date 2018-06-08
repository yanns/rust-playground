extern crate actix_web;

use actix_web::{http, server, App, HttpRequest};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};


// This struct represents state
struct AppState {
    counter: Arc<AtomicUsize>,
}

fn index(req: HttpRequest<AppState>) -> String {
    let count = req.state().counter.fetch_add(1, Ordering::Relaxed);

    format!("Request number: {}", count + 1)
}

fn new_app(counter: Arc<AtomicUsize>) -> App<AppState> {
    App::with_state(AppState {
        counter: counter,
    }).resource("/", |r| r.method(http::Method::GET).f(index))
}

fn main() {
    let counter = Arc::new(AtomicUsize::new(0));
    server::new(move || new_app(counter.clone()))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}
