use std::error::Error;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::io::stdin;
use std::fs::File;
use std::collections::HashMap;
use std::iter::FromIterator;

fn sort_string(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().into_iter().collect();
    chars.sort();
    String::from_iter(chars.into_iter())
}

fn main() {
    println!("Parsing data...");
    let path = Path::new("../../../../words.txt");
    let f = match File::open(path) {
        Ok(f) => f,
        Err(why) => panic!("Cannot open file {}: {}", path.display(), why.description()),
    };
    let file = BufReader::new(&f);

    let mut dict: HashMap<String, Vec<String>> = HashMap::new();

    for line in file.lines() {
        let l = line.unwrap();
        dict.entry(sort_string(&l)).or_insert(Vec::new()).push(l);
    }

    println!("Enter your word: ");
    let sin = stdin();
    for line in sin.lock().lines() {
        let l = line.unwrap();

        match dict.get(&sort_string(&l)) {
            Some(words) => println!("Anagrams for {}: {}", &l, words.join(" ")),
            None => println!("Sorry, found no word for {}", l),
        }
        println!("Enter your word: ");
    }

}
