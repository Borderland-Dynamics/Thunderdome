use std::io::{self, Write};

struct Damage {
    amount: u32,
    overkill: u32,
}

struct Character {
    name: String,
    stamina: u32,
    strength: u32,
}

impl Character {
    fn damage(&mut self, amount: u32) -> Damage {
        let mut overkill = 0;
        let mut damage = amount.clone();

        if self.stamina >= amount {
            self.stamina = self.stamina - amount;
        } else {
            overkill = amount - self.stamina;
            damage = self.stamina.clone();
            self.stamina = 0;
        }

        Damage { amount: damage, overkill }
    }

    fn attack(&self, target: &mut Character) -> Damage {
        target.damage(self.strength)
    }
}

struct LocalPlayer {}

impl LocalPlayer {
    fn tell(&self, what: String) {
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
}

fn main() {
    let player = LocalPlayer {};
    let name = player.ask(String::from("Gladiator! What is your name?"));

    let mut player_character = Character {
        name,
        stamina: 25,
        strength: 5,
    };

    let mut monster = Character {
        name: String::from("Monster"),
        stamina: 50,
        strength: 15,
    };

    player.tell(String::from("\nListen all! This is the truth of it.\n"));
    player.tell(String::from("Fighting leads to killing, and killing gets to warring. And that was damn near the death of us all. Look at us now! Busted up, and everyone talking about hard rain! But we’ve learned, by the dust of them all… Bartertown learned.\n"));
    player.tell(String::from("Now, when men get to fighting, it happens here! And it finishes here!\n"));
    player.tell(String::from("Two men enter; one man leaves.\n"));

    loop {
        let monster_damage = monster.attack(&mut player_character);
        player.tell(format!("{} attacks {} for {} damage!", monster.name, player_character.name, monster_damage.amount));
        if player_character.stamina == 0 && monster_damage.overkill > 0 {
            player.tell(format!("{} is dead. ({} overkill)", player_character.name, monster_damage.overkill));
            break
        } else if player_character.stamina == 0 {
            player.tell(format!("{} is dead.", player_character.name));
            break
        } else {
            player.tell(format!("{} now has {} stamina.", player_character.name, player_character.stamina));
        }

        let player_damage = player_character.attack(&mut monster);
        player.tell(format!("{} attacks {} for {} damage!", player_character.name, monster.name, player_damage.amount));
        if monster.stamina == 0 && player_damage.overkill > 0 {
            player.tell(format!("{} is dead. ({} overkill)", monster.name, player_damage.overkill));
            break
        } else if monster.stamina == 0 {
            player.tell(format!("{} is dead.", monster.name));
            break
        } else {
            player.tell(format!("{} now has {} stamina.", monster.name, monster.stamina));
        }
    }

    if player_character.stamina <= 0 && monster.stamina > 0 {
        player.tell(format!("{} wins!", monster.name));
    } else if monster.stamina <= 0 && player_character.stamina > 0 {
        player.tell(format!("{} wins!", player_character.name));
    } else {
        player.tell(String::from("Well, I guess sometimes two men enter, nobody leaves."));
    }
}
