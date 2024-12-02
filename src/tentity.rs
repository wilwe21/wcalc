use crate::save;
use crate::bag::Item;
use crate::bag::Bag;

#[derive(Clone, Debug)]
pub struct Entity {
    pub display: char,
    pub character: String,
    pub attacks: Vec<String>,
    pub position: String,
    pub health: usize,
    pub maxhealth: usize,
    pub bag: Option<Bag>,
    pub status: Option<Vec<String>>,
    pub lvl: usize,
    pub score: usize,
    pub room: String,
    pub mode: String
}

impl Entity {
    pub fn new_player(character: String, position: String, room: String) -> Self {
        let attacks: Vec<String> = vec!("bite".to_string(), "divide".to_string(), "".to_string(), "".to_string());
        let potion = Item::get_by_id("potionHP").unwrap();
        let bag = Bag::new(Some((potion, 5)), None, None, None);
        let health: usize = 100;
        Self {
            display: 'P',
            character,
            attacks,
            position,
            health,
            maxhealth: health,
            bag: Some(bag),
            status: None,
            lvl: 0,
            score: 0,
            room: "0".to_string(),
            mode: "None".to_string()
        }
    }

    pub fn new(character: String, attacks: Vec<String>, health: usize, lvl: usize) -> Self {
        let display = character.chars().collect::<Vec<char>>()[0];
        Self {
            display,
            character,
            attacks,
            position: "None".to_string(),
            health,
            maxhealth: health,
            bag: None,
            status: None,
            lvl,
            score: 0,
            room: "None".to_string(),
            mode: "None".to_string()
        }
    }

    pub fn enemy_list() -> Vec<Self> {
        let snake = Self::new("Snake".to_string(), vec!("bite".to_string(),"venom".to_string(), "".to_string(),"".to_string()), 100, 1);
        let rock = Self::new("Rock".to_string(), vec!("standStill".to_string(),"".to_string(), "".to_string(),"".to_string()), 5, 0);
        let horse = Self::new("Horse".to_string(), vec!("kick".to_string(), "standStill".to_string(), "standStill".to_string(),"".to_string()), 100, 1);
        return vec!(snake, rock, horse)
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

    pub fn get_by_display(dis: &str) -> Option<Self> {
        let list = Self::enemy_list();
        for i in list {
            if i.display.to_string() == dis.to_string() {
                return Some(i)
            }
        }
        None
    }

    pub fn from_str(st: String) -> Self {
        let s = save::str_to_conf(st);
        let s = s.get("player").unwrap();
        let character = s.get("character").unwrap().to_string();
        let display = character.chars().collect::<Vec<char>>()[0];
        let a1 = s.get("attack1").unwrap().to_string();
        let a2 = s.get("attack2").unwrap().to_string();
        let a3 = s.get("attack3").unwrap().to_string();
        let a4 = s.get("attack4").unwrap().to_string();
        let attacks: Vec<String> = vec!(a1,a2,a3,a4);
        let position = s.get("position").unwrap().to_string();
        let health = s.get("health").unwrap().parse::<usize>().unwrap();
        let maxhealth = s.get("maxhealth").unwrap().parse::<usize>().unwrap();
        let bag = s.get("bag").unwrap().parse::<bool>().unwrap();
        let baggy = Bag::new(None, None, None, None);
        if bag {
            let s0v = s.get("item0").unwrap();
            let s0: Option<(Item, usize)> = None;
            if s0v != "None" {
                let sus = s0v.split(", ").collect::<Vec<_>>();
                let is = Item::get_by_id(sus[0]);
                let am = sus[1].parse::<usize>();
                let s0 = Some((is, am));
            } else {
                let s0: Option<(Item, usize)> = None;
            }
            let s1v = s.get("item0").unwrap();
            let s1: Option<(Item, usize)> = None;
            if s1v != "None" {
                let sus = s1v.split(", ").collect::<Vec<_>>();
                let is = Item::get_by_id(sus[0]);
                let am = sus[1].parse::<usize>();
                let s1 = Some((is, am));
            } else {
                let s1: Option<(Item, usize)> = None;
            }
            let s2v = s.get("item0").unwrap();
            let s2: Option<(Item, usize)> = None;
            if s2v != "None" {
                let sus = s2v.split(", ").collect::<Vec<_>>();
                let is = Item::get_by_id(sus[0]);
                let am = sus[1].parse::<usize>();
                let s2 = Some((is, am));
            } else {
                let s2: Option<(Item, usize)> = None;
            }
            let s3v = s.get("item3").unwrap();
            let s3: Option<(Item, usize)> = None;
            if s3v != "None" {
                let sus = s3v.split(", ").collect::<Vec<_>>();
                let is = Item::get_by_id(sus[0]);
                let am = sus[1].parse::<usize>();
                let s3 = Some((is, am));
            } else {
                let s3: Option<(Item, usize)> = None;
            }
            let baggy = Bag::new(s0, s1, s2, s3);
        }
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
            display,
            character,
            attacks,
            position,
            health,
            maxhealth,
            bag: Some(baggy),
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
        st += &format!("bag = ");
        match self.bag {
            Some(b) => {
                st += &format!("true\n");
                for j in 0..4 {
                    st += &format!("item{} = ", j);
                    match b.get_by_id(j) {
                        Some(bi) => {
                            st += &format!("{}, {}\n", bi.0.id, bi.1);
                        },
                        _ => st += &format!("None\n"),
                    }
                }
            },
            _ => st += &format!("false\n")
        }
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
    pub fn heal(&mut self, amount: usize) {
        if self.health + amount > self.maxhealth {
            self.health = self.maxhealth;
        } else {
            self.health += amount;
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
    pub fn change_bag(&mut self, bag: Bag) {
        self.bag = Some(bag);
    }
}
