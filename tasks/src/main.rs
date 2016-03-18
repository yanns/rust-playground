extern crate rand;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::time::Duration;
use std::thread;
use rand::random;

fn monte_carlo_pi(n: usize, sender: Sender<usize>) {
	println!("montecarlopi(): Starting calculation");
	let mut m = 0usize;
	for _ in 0usize..n {
		let x = random::<f32>();
		let y = random::<f32>();
		if (x*x + y*y) < 1.0 {
			m = m + 1;
		}
	}
	println!("montecarlopi(): Calculation done");
	// do not panic if cannot send
	sender.send(m).ok();
}

fn worker(receiver: Receiver<usize>, send_to_main: Sender<f64>) {
	let mut m = 0usize;
	let n = 10_000_000;
	let mut i = 0;
	let (sender, receive_from_montecarlo) = mpsc::channel();
	let initial_sender = sender.clone();
	thread::spawn(move || monte_carlo_pi(n, initial_sender));
	loop {
		if receiver.try_recv().is_ok() {
			println!("worker(): Aborting calculation due to signal from main (i={})", i);
			break;
		}
		if let Ok(r) = receive_from_montecarlo.try_recv() {
            m = m + r;
            i = i + 1;
            let sender_clone = sender.clone();
            thread::spawn(move || monte_carlo_pi(n, sender_clone));
        }
        // main can interrupt worker every 50 ms
        thread::sleep(Duration::from_millis(50));
	}
	let val = 4.0 * (m as f64) / ((n*i) as f64);
    send_to_main.send(val).unwrap();
}

fn main() {
	let wait_in_s = 10;

	// channel from worker to main to send the value of PI
	let (send_from_worker_to_main, receive_from_worker) = mpsc::channel();

	// channel from main to worker to signal a stop
	let (send_from_main_to_worker, receive_from_main) = mpsc::channel();

	println!("main(): start calculation and wait {}s", wait_in_s);
    thread::spawn(|| worker(receive_from_main, send_from_worker_to_main));

    thread::sleep(Duration::from_secs(wait_in_s));
    println!("main(): Sending abort to worker");

    send_from_main_to_worker.send(0).unwrap();
    let val = receive_from_worker.recv().unwrap();
    println!("main(): pi = {}. Error = {}", val, (std::f64::consts::PI - val).abs());
}
