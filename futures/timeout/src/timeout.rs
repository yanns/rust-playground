use std::io;
use std::time::{Duration, Instant};
use std::marker::PhantomData;

use futures::{Future, Poll};

pub struct Timeout<T, E> {
    timestamp: Instant,
    duration: Duration,
    phantom: PhantomData<T>,
    error: E,
}

impl<T, E> Timeout<T, E>
    where E: Fn() -> io::Error
{
    pub fn new(duration: Duration, e: E) -> Timeout<T, E> {
        Timeout {
            timestamp: Instant::now(),
            duration: duration,
            phantom: PhantomData,
            error: e,
        }
    }

    pub fn is_elapsed(&self) -> bool {
        self.timestamp.elapsed() >= self.duration
    }
}


impl<T, E> Future for Timeout<T, E>
    where E: Fn() -> io::Error
{
    type Item = T;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use futures::Async;
        use futures::task;
        // println!("poll!");

        if self.is_elapsed() {
            Err((self.error)())
        } else {
            task::park().unpark();
            Ok(Async::NotReady)
        }
    }
}
