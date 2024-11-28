use crate::save;

#[derive(Clone, Debug)]
pub struct Player {
    character: String,
    attacks: Vec<String>,
    position: String,
    health: usize,
    lvl: usize,
    score: usize,
    room: String
}

impl Player {
    pub fn new(character: String, position: String, room: String) -> Self {
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

    pub fn from_str(st: String) -> Self {
        let s = save::str_to_conf(st);
        let s = s.get("player").unwrap();
        let character = s.get("character").unwrap().to_string();
        let a1 = s.get("attack1").unwrap().to_string();
        let a2 = s.get("attack2").unwrap().to_string();
        let a3 = s.get("attack3").unwrap().to_string();
        let a4 = s.get("attack4").unwrap().to_string();
        let attacks: Vec<String> = vec!(a1,a2,a3,a4);
        let position = s.get("position").unwrap().to_string();
        let health = s.get("health").unwrap().parse::<usize>().unwrap();
        let lvl = s.get("lvl").unwrap().parse::<usize>().unwrap();
        let score = s.get("score").unwrap().parse::<usize>().unwrap();
        let room = s.get("room").unwrap().to_string();
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
