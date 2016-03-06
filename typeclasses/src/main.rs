trait ToJson {
	fn to_json(&self) -> String;
}

impl ToJson for str {
	fn to_json(&self) -> String {
		format!("\"{}\"", &self)
	}
}

// impl<'a> ToJson for &'a str {
// 	fn to_json(&self) -> String {
// 		format!("\"{}\"", &self)
// 	}
// }

// more general approach than &str:
// for all &T that implements ToJson.
// The size is optionaly known at compile time to be able to use ToJson<str> for example.
// applies to &str can use ToJson<str>
impl<'a, T: ?Sized + ToJson> ToJson for &'a T {
	fn to_json(&self) -> String {
		(*self).to_json()
	}
}

impl ToJson for String {
	fn to_json(&self) -> String {
		format!("\"{}\"", &self)
	}
}

impl ToJson for i32 {
	fn to_json(&self) -> String {
		format!("{}", &self)
	}	
}

impl<T: ToJson> ToJson for Vec<T> {
	fn to_json(&self) -> String {
		let mut result = String::new();
		result = result + "[";
		let mut first = true;
		for i in self {
			result = if first {
				first = false;
				result + &i.to_json()
			} else {
				result + &", ".to_string() + &i.to_json()
			}
		}
		result + "]"
	}		
}

#[derive(Debug)]
struct Person {
	name: String,
	age: i32,
}
impl Person {
	fn new(name: &str, age: i32) -> Person {
		Person { name: name.to_string(), age: age }
	}
}

impl ToJson for Person {
	fn to_json(&self) -> String {
		format!(r#"{{
  "name": {},
  "age": {}
}}"#, self.name.to_json(), self.age.to_json())
	}	
}

fn main() {
	println!("{}", "hello world".to_json());
	println!("{}", 3.to_json());
	println!("{}", vec![1, 2, 3, 4, 5].to_json());
	println!("{}", vec!["hello".to_string(), "world".to_string()].to_json());
	println!("{}", vec!["hello", "world"].to_json());

	let p1 = Person::new("yann", 38);
	println!("{}", p1.to_json());
	println!("{}", vec![p1, Person::new("paul", 58)].to_json());
}
