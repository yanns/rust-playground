#[macro_use]
extern crate nickel;
use nickel::{Nickel, HttpRouter};

fn main() {
    let mut app = Nickel::new();
    app.get("/", middleware!("Hello World!"));
    app.listen("localhost:3000")
}
