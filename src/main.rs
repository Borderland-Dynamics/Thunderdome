use std::io;

struct Damage {
    amount: u32,
    overkill: u32,
}

struct Character {
    name: String,
    stamina: u32,
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
        target.damage(5)
    }
}

struct LocalPlayer {}

impl LocalPlayer {
    fn tell(&self, what: String) {
        println!("{}", what);
    }

    fn ask(&self, question: String) -> String {
        let mut answer = String::new();

        println!("{}", question);

        io::stdin()
            .read_line(&mut answer)
            .expect("Error reading player input.");

        answer.trim().to_string()
    }
}

fn main() {
    let player = LocalPlayer {};
    let name = player.ask(String::from("Gladiator! What is your name?"));

    let mut player_character = Character { name, stamina: 30 };

    let mut monster = Character {
        name: String::from("Monster"),
        stamina: 20,
    };

    player.tell(String::from("Two men enter, one man leaves!"));

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
