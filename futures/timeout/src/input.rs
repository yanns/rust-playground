use futures::*;
use std::io;
use std::thread;

pub struct ReadLine {
    recv: Oneshot<String>,
}

impl ReadLine {
    pub fn new() -> ReadLine {
        let (tx, rx) = oneshot::<String>();
        thread::spawn(move || read_line(tx));
        ReadLine { recv: rx }
    }
}

impl Future for ReadLine {
    type Item = String;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        println!("poll!");
        match self.recv.poll() {
            Err(_) => Err(io::Error::new(io::ErrorKind::Other, "blabla")),
            Ok(a) => Ok(a),
        }
    }
}

fn read_line(tx: Complete<String>) {
    use std::io::BufRead;

    let input = io::stdin();
    let mut locked = input.lock();
    let mut buf = String::new();

    match locked.read_line(&mut buf) {
        Ok(_) => tx.complete(buf),
        Err(e) => panic!(e),
    }
}
