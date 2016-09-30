extern crate futures;

mod input;
mod timeout;

use std::io;
use futures::Future;
use timeout::Timeout;
use std::time::Duration;
use input::ReadLine;

fn main() {
    match read_name() {
        Err(_) => println!("Hello, whatever your name is"),
        Ok(name) => println!("Hello {}", name.trim()),
    }
}

fn read_name() -> io::Result<String> {
    let result = ReadLine::new()
        .select(Timeout::new(Duration::from_secs(3), || {
            io::Error::new(io::ErrorKind::Other, "timeout elapsed".to_string())
        }))
        .wait();

    match result {
        Ok((name, _)) => Ok(name),
        Err((e, _)) => Err(e),
    }
}
