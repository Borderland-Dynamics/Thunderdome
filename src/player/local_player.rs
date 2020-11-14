use std::io::{self, Write};

use super::super::combat::Damage;

pub struct LocalPlayer {}

impl LocalPlayer {
    pub fn tell(&self, what: String) {
        println!("{}", what);
    }

    fn ask(&self, question: String) -> String {
        let mut answer = String::new();

        print!("{} ", question);
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut answer)
            .expect("Error reading player input.");

        answer.trim().to_string()
    }

    pub fn ask_name(&self) -> String {
        self.ask(String::from("Gladiator! What is your name?"))
    }

    pub fn tell_prologue(&self) {
        self.tell(String::from("Listen all! This is the truth of it.\n
Fighting leads to killing, and killing gets to warring. And that was damn near the death of us all. Look at us now! Busted up, and everyone talking about hard rain! But we’ve learned, by the dust of them all… Bartertown learned.\n
Now, when men get to fighting, it happens here! And it finishes here!\n
Two men enter; one man leaves.\n"));
    }

    pub fn tell_damage_report(&self, damage: Damage) {
        let Damage {
            attacker_name,
            defender_name,
            total,
            overkill,
            remaining,
        } = damage;

        self.tell(format!(
            "{} attacks {} for {} damage!",
            attacker_name, defender_name, total
        ));

        if remaining == 0 && overkill > 0 {
            self.tell(format!( "{} is dead. ({} overkill)",
                defender_name, overkill
            ));
        } else if remaining == 0 {
            self.tell(format!("{} is dead.", defender_name));
        } else {
            self.tell(format!("{} now has {} stamina.", defender_name, remaining));
        }
    }
}
