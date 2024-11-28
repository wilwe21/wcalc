#[derive(Clone, Debug)]
pub struct Player {
    character: usize,
    attacks: Vec<String>,
    position: String,
    health: usize,
    lvl: usize,
    score: usize,
    room: String
}

impl Player {
    pub fn new(character: usize, position: String, room: String) -> Self {
        let attacks: Vec<String> = vec!("dupa".to_string(), "kutas".to_string(), "".to_string(), "".to_string());
        let health: usize = 100;
        let lvl: usize = 0;
        let score: usize = 0;
        Self {
            character,
            attacks,
            position,
            health,
            lvl,
            score,
            room
        }
    }
    pub fn to_string(self) -> String {
        let mut st = "[player]\n".to_string();
        st += &format!("character = {}\n", self.character);
        st += &format!("attack1 = {}\n", self.attacks[0]);
        st += &format!("attack2 = {}\n", self.attacks[1]);
        st += &format!("attack3 = {}\n", self.attacks[2]);
        st += &format!("attack4 = {}\n", self.attacks[3]);
        st += &format!("position = {}\n", self.position);
        st += &format!("health = {}\n", self.health);
        st += &format!("lvl = {}\n", self.lvl);
        st += &format!("score = {}\n", self.score);
        st += &format!("room = {}\n", self.room);
        return st
    }
    pub fn get_dmg(&mut self, amount: usize) {
        self.health -= amount;
    }
    pub fn move_to(&mut self, position: String) {
        self.position = position;
    }
    pub fn move_room(&mut self, room: String) {
        self.room = room;
    }
}
