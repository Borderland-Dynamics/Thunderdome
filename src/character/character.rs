use super::super::combat::Damage;
use super::super::random::roll;

pub struct Character {
    pub name: String,
    pub stamina: u32,
    pub strength: u32,
}

impl Character {
    pub fn new(name: String, stamina: u32, strength: u32) -> Character {
        Character {
            name,
            stamina,
            strength,
        }
    }

    pub fn damage(&mut self, total: u32) -> Damage {
        let mut overkill = 0;

        if self.stamina >= total {
            self.stamina = self.stamina - total;
        } else {
            overkill = total - self.stamina;
            self.stamina = 0;
        }

        Damage { total, overkill }
    }

    pub fn attack(&self, target: &mut Character) -> Damage {
        target.damage(self.strength + roll(6) + roll(6) - 6)
    }
}
