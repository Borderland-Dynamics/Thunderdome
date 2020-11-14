mod character;
mod combat;
mod player;
mod random;

use character::Character;
use combat::Damage;
use player::LocalPlayer;

fn main() {
    let player = LocalPlayer {};
    let name = player.ask_name();

    let mut player_character = Character::new(name, 25, 5);
    let mut monster = Character::new(String::from("Master Blaster"), 35, 8);

    player.tell_prologue();

    loop {
        let mut damage: Damage;
        damage = monster.attack(&mut player_character);
        player.tell_damage_report(damage);
        if player_character.stamina <= 0 {
            break;
        }

        damage = player_character.attack(&mut monster);
        player.tell_damage_report(damage);
        if monster.stamina <= 0 {
            break;
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
