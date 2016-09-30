use futures::*;
use futures::task::Task;
use std::io;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

pub struct ReadLine {
    recv: Receiver<io::Result<String>>,
    taskSender: Sender<Task>,
}

impl ReadLine {
    pub fn new() -> ReadLine {
        let (tx, rx) = mpsc::channel();
        let (txTask, rxTask) = mpsc::channel();
        thread::spawn(move || tx.send(read_line(rxTask)));
        ReadLine { recv: rx, taskSender: txTask }
    }
}

impl Future for ReadLine {
    type Item = String;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use futures::Async;
        use futures::task;

        match self.recv.try_recv() {
            Err(_) => {
                let task = task::park();
                self.taskSender.send(task).unwrap();
                Ok(Async::NotReady)
            }
            Ok(Ok(line)) => Ok(Async::Ready(line)),
            Ok(Err(e)) => Err(e),

        }
    }
}

fn read_line(rx: Receiver<Task>) -> io::Result<String> {
    use std::io::BufRead;

    let input = io::stdin();
    let mut locked = input.lock();
    let mut buf = String::new();
    let task = rx.recv().unwrap();

    match locked.read_line(&mut buf) {
        Ok(_) => {
            task.unpark();
            Ok(buf)
        }
        Err(e) => {
            task.unpark();
            Err(e)
        }
    }
}
