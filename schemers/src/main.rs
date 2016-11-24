extern crate rustyline;

fn main() {
    let mut done = false;
    let mut reader = rustyline::Editor::<()>::new();
    while !done {
        match reader.readline(">> ") {
            Ok(line) => {
                if line.trim() == "(exit)" {
                    done = true;
                } else {
                    println!("{}", line);
                }
            }
            Err(e) => {
                use rustyline::error::ReadlineError::*;
                match e {
                    Eof | Interrupted => done = true,
                    _ => println!("Couldn't readline. Error was: {}", e),
                }
            }
        }
    }
}
