use crate::save;

#[derive(Clone, Debug)]
pub struct Entity {
    pub character: String,
    pub attacks: Vec<String>,
    pub position: String,
    pub health: usize,
    pub maxhealth: usize,
    pub status: Option<Vec<String>>,
    pub lvl: usize,
    pub score: usize,
    pub room: String,
    pub mode: String
}

impl Entity {
    pub fn new_player(character: String, position: String, room: String) -> Self {
        let attacks: Vec<String> = vec!("Bite".to_string(), "Divide".to_string(), "".to_string(), "".to_string());
        let health: usize = 100;
        Self {
            character,
            attacks,
            position,
            health,
            maxhealth: health,
            status: None,
            lvl: 0,
            score: 0,
            room: "0".to_string(),
            mode: "None".to_string()
        }
    }

    pub fn new(character: String, attacks: Vec<String>, health: usize, lvl: usize) -> Self {
        Self {
            character,
            attacks,
            position: "None".to_string(),
            health,
            maxhealth: health,
            status: None,
            lvl,
            score: 0,
            room: "None".to_string(),
            mode: "None".to_string()
        }
    }

    pub fn enemy_list() -> Vec<Self> {
        let snake = Self::new("Snake".to_string(), vec!("Bite".to_string(),"Venom".to_string(), "".to_string(),"".to_string()), 100, 1);
        return vec!(snake)
    }

    pub fn get_by_name(name: &str) -> Option<Self> {
        let list = Self::enemy_list();
        for i in list {
            if i.character == name.to_string() {
                return Some(i)
            }
        }
        None
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
        let maxhealth = s.get("maxhealth").unwrap().parse::<usize>().unwrap();
        let stat = s.get("status").unwrap();
        let status = None;
        if stat != "None" {
            let status = Some(stat.split(", ").map(|r| r.to_string()).collect::<Vec<String>>());
        }
        let lvl = s.get("lvl").unwrap().parse::<usize>().unwrap();
        let score = s.get("score").unwrap().parse::<usize>().unwrap();
        let room = s.get("room").unwrap().to_string();
        let mode = s.get("mode").unwrap().to_string();
        Self {
            character,
            attacks,
            position,
            health,
            maxhealth,
            status,
            lvl,
            score,
            room,
            mode
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
        st += &format!("maxhealth = {}\n", self.maxhealth);
        st += &format!("status = ");
        match self.status {
            Some(s) => {
                for (j, i) in s.into_iter().enumerate() {
                    if j != 0 {
                        st += &format!("{}, ",i);
                    } else {
                        st += &format!("{}",i);
                    }
                }
                st += "\n";
            },
            _ => st += &format!("None\n")
        };

        st += &format!("maxhealth = {}\n", self.maxhealth);
        st += &format!("lvl = {}\n", self.lvl);
        st += &format!("score = {}\n", self.score);
        st += &format!("room = {}\n", self.room);
        st += &format!("mode = {}\n", self.mode);
        return st
    }
    pub fn get_dmg(&mut self, amount: usize) {
        if self.health < amount {
            self.health = 0;
        } else {
            self.health -= amount;
        }
    }
    pub fn move_to(&mut self, position: String) {
        self.position = position;
    }
    pub fn move_room(&mut self, room: String) {
        self.room = room;
    }
    pub fn change_mode(&mut self, mode: String) {
        self.mode = mode;
    }
    pub fn apply_status(&mut self, status: String) {
        match &self.status {
            Some(_) => {
                let mut st = self.status.clone().unwrap();
                st.push(status);
                self.status = Some(st)
            },
            _ => self.status = Some(vec!(status))
        }
    }
}
