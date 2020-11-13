use std::io::{self, Write};

pub struct LocalPlayer {}

impl LocalPlayer {
    pub fn tell(&self, what: String) {
        println!("{}", what);
    }

    pub fn ask(&self, question: String) -> String {
        let mut answer = String::new();

        print!("{} ", question);
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut answer)
            .expect("Error reading player input.");

        answer.trim().to_string()
    }
}
