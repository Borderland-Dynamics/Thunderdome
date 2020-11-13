mod character;
mod combat;
mod player;
mod random;

use character::Character;
use player::LocalPlayer;

fn main() {
    let player = LocalPlayer {};
    let name = player.ask(String::from("Gladiator! What is your name?"));

    let mut player_character = Character::new(name, 25, 5);
    let mut monster = Character::new(String::from("Master Blaster"), 35, 8);

    player.tell(String::from("\nListen all! This is the truth of it.\n"));
    player.tell(String::from("Fighting leads to killing, and killing gets to warring. And that was damn near the death of us all. Look at us now! Busted up, and everyone talking about hard rain! But we’ve learned, by the dust of them all… Bartertown learned.\n"));
    player.tell(String::from(
        "Now, when men get to fighting, it happens here! And it finishes here!\n",
    ));
    player.tell(String::from("Two men enter; one man leaves.\n"));

    loop {
        let monster_damage = monster.attack(&mut player_character);
        player.tell(format!(
            "{} attacks {} for {} damage!",
            monster.name, player_character.name, monster_damage.total
        ));
        if player_character.stamina == 0 && monster_damage.overkill > 0 {
            player.tell(format!(
                "{} is dead. ({} overkill)",
                player_character.name, monster_damage.overkill
            ));
            break;
        } else if player_character.stamina == 0 {
            player.tell(format!("{} is dead.", player_character.name));
            break;
        } else {
            player.tell(format!(
                "{} now has {} stamina.",
                player_character.name, player_character.stamina
            ));
        }

        let player_damage = player_character.attack(&mut monster);
        player.tell(format!(
            "{} attacks {} for {} damage!",
            player_character.name, monster.name, player_damage.total
        ));
        if monster.stamina == 0 && player_damage.overkill > 0 {
            player.tell(format!(
                "{} is dead. ({} overkill)",
                monster.name, player_damage.overkill
            ));
            break;
        } else if monster.stamina == 0 {
            player.tell(format!("{} is dead.", monster.name));
            break;
        } else {
            player.tell(format!(
                "{} now has {} stamina.",
                monster.name, monster.stamina
            ));
        }
    }

    if player_character.stamina <= 0 && monster.stamina > 0 {
        player.tell(format!("{} wins!", monster.name));
    } else if monster.stamina <= 0 && player_character.stamina > 0 {
        player.tell(format!("{} wins!", player_character.name));
    } else {
        player.tell(String::from(
            "Well, I guess sometimes two men enter, nobody leaves.",
        ));
    }
}
