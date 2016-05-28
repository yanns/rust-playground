pub fn strings() {
	let s1 = "hello";
	let s2 = "hello";
	println!("s1 == s2: {:?}", s1 == s2); // true

	let s3 = "h".to_string() + "ello";
	println!("s1 == s3: {:?}", s1 == s3); // true

	let multi = "This is
	a multi line
	string
	";
	println!("multi = {:?}", multi); // "This is\n\ta multi line\n\tstring\n\t"

	let multi2 = "This is \
	a multi line \
	string \
	";
	println!("multi2 = {:?}", multi2); // "This is a multi line string "

	let multi3 = r#"
	{
		"name": "john",
		"age" 42
	}
	"#;
	println!("multi3 = {:?}", multi3); // "\n\t{\n\t\t\"name\": \"john\",\n\t\t\"age\" 42\n\t}\n\t"

	let name = "john";
	let age = 42;
	let multi4 = format!(r#"
	{{
		"name": "{}",
		"age" {}
	}}
	"#, name, age);
	println!("multi4 = {:?}", multi4); // "\n\t{\n\t\t\"name\": \"john\",\n\t\t\"age\" 42\n\t}\n\t"
}