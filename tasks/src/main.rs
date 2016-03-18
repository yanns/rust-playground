extern crate rand;

use rand::random;

fn monte_carlo_pi(n: usize) -> f32 {
	let mut m = 0usize;
	for _ in 0usize..n {
		let x = random::<f32>();
		let y = random::<f32>();
		if (x*x + y*y) < 1.0 {
			m = m + 1;
		}
	}
	4.0 * (m as f32) / (n as f32)
}

fn main() {
    println!("For       1000 random drawings pi = {}", monte_carlo_pi(1000));
    println!("For      10000 random drawings pi = {}", monte_carlo_pi(10000));
    println!("For     100000 random drawings pi = {}", monte_carlo_pi(100000));
    println!("For    1000000 random drawings pi = {}", monte_carlo_pi(1000000));
    println!("For   10000000 random drawings pi = {}", monte_carlo_pi(10000000));
}
