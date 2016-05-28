pub fn firststeps() {
	// type inference
	let msg = "Hello, world!";
	println!("msg = {:?}", msg);

	// explicit type
	let msg2: &str = "Hello, world!";
	println!("msg2 = {:?}", msg2);

	let msg3: String = "Hello, world!".to_string();
	println!("ms3 = {:?}", msg3);

	// mutation
	let mut greeting = "Hello, world!";
	println!("greeting = {:?}", greeting);
	greeting = "Leave me alone, world!";
	println!("greeting = {:?}", greeting);

	// functions
	fn max(x: i32, y: i32) -> i32 {
		if x > y { x } else { y }
	}
	println!("max(2, 4) = {:?}", max(2, 4));
	println!("max(2, -4) = {:?}", max(2, -4));
	println!("max(-2, -4) = {:?}", max(-2, -4));

	// procedure
	fn greet() {
		println!("greet!");
	}
	greet();

	// unit type
	fn greet2() -> () {
		println!("greet2!");
	}
	greet2();
}