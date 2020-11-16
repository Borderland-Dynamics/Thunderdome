use super::super::combat::Damage;
use super::super::random::roll;

pub struct Character {
    pub name: String,
    pub stamina: u32,
    pub strength: u32,
}

impl Character {
    pub fn new(name: &str, stamina: u32, strength: u32) -> Character {
        Character {
            name: name.to_string(),
            stamina,
            strength,
        }
    }

    pub fn damage(&mut self, attacker: &Character, total: u32) -> Damage {
        let mut overkill = 0;

        if self.stamina >= total {
            self.stamina = self.stamina - total;
        } else {
            overkill = total - self.stamina;
            self.stamina = 0;
        }

        Damage {
            attacker_name: attacker.name.clone(),
            defender_name: self.name.clone(),
            total,
            overkill,
            remaining: self.stamina,
        }
    }

    pub fn attack(&self, target: &mut Character) -> Damage {
        target.damage(&self, self.strength + roll(6) + roll(6) - 6)
    }
}
