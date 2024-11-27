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
        let attacks: Vec<String> = vec!();
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
