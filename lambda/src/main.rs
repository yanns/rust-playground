fn main() {
	let v = vec!(1, 2 ,3);
	let v2: Vec<_> = v.iter().map(|n| n / 0).collect();
    println!("v2 = {:#?}", v2);
}
