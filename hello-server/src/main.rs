extern crate nickel;
use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult};

fn hello_world<'mv>(_req: &mut Request, res: Response<'mv>) -> MiddlewareResult<'mv> {
    res.send("Hello World!")
}

fn main() {
    let mut app = Nickel::new();
    app.get("/", hello_world);
    app.listen("localhost:3000")
}
