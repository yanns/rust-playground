use futures::*;
use std::io;
use std::thread;

pub struct ReadLine {
    recv: Oneshot<io::Result<String>>,
}

impl ReadLine {
    pub fn new() -> ReadLine {
        let (tx, rx) = oneshot::<io::Result<String>>();
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
            Err(_) => Err(io::Error::new(io::ErrorKind::Other, "future cancelled")),
            Ok(Async::Ready(Ok(line))) => Ok(Async::Ready(line)),
            Ok(Async::Ready(Err(e))) => Err(e),
            Ok(Async::NotReady) => Ok(Async::NotReady),
        }
    }
}

fn read_line(tx: Complete<io::Result<String>>) {
    use std::io::BufRead;

    let input = io::stdin();
    let mut locked = input.lock();
    let mut buf = String::new();

    match locked.read_line(&mut buf) {
        Ok(_) => tx.complete(Ok(buf)),
        Err(e) => tx.complete(Err(e)),
    }
}
