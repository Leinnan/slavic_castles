pub struct Player {
    pub human: bool,
    pub alive: bool,
    pub id: i32,
    pub tower_hp: i32,
    pub walls_hp: i32,
}

impl Player {
    pub fn new(new_id: i32) -> Player {
        Player {
            human: true,
            alive: true,
            id: new_id,
            tower_hp: 20,
            walls_hp: 15,
        }
    }

    pub fn make_tower_higher(&mut self, amount: i32) {
        self.tower_hp += amount;
    }

    pub fn give_damage(&mut self, amount: i32, ignore_wall: bool) {
        if ignore_wall {
            self.tower_hp -= amount;
        }

        if self.walls_hp < amount {
            self.tower_hp -= amount - self.walls_hp;
            self.walls_hp = 0;
        } else {
            self.tower_hp -= amount;
            if self.tower_hp < 0 {
                self.tower_hp = 0;
            }
        }

        if self.tower_hp == 0 {
            self.alive = false;
        }
    }
}
