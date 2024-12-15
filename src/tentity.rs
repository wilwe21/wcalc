use std::fmt;
use std::path::Path;

use crate::save;
use crate::bag::Item;
use crate::bag::Bag;
use crate::game;
use crate::tattacks::Attack;

use crate::conf;

#[derive(Clone, Debug)]
pub struct Entity {
    pub display: char,
    pub image: String,
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
    pub floor: Option<usize>,
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "Name: {}, HP: {}/{}", self.character, self.health, self.maxhealth)
        } else {
            write!(f, "{}, {}/{}", self.character, self.health, self.maxhealth)
        }
    }
}

pub fn check_player_sprite(sprite: &str) -> String {
    let path = format!("{}/entity/{}_player.png", conf::assets_path().unwrap(), sprite);
    let p = Path::new(&path);
    if p.is_file() {
        return path.to_string();
    } else {
        return format!("{}/entity/{}.png", conf::assets_path().unwrap(), sprite)
    }
}

impl Entity {
    pub fn new_player(character: String, image: Option<String>, position: String) -> Self {
        let mut im = String::new();
        match image {
            Some(s) => im = check_player_sprite(&s),
            _ => im = check_player_sprite("one")
        };
        let attacks: Vec<String> = vec!("bite".to_string(), "divide".to_string(), "".to_string(), "".to_string());
        let potion = Item::get_by_id("potionHP").unwrap();
        let bag = Bag::new(Some((potion, 5)), None, None, None);
        let health: usize = 100;
        Self {
            display: '1',
            image: im,
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
            floor: Some(0),
        }
    }
    
    pub fn player_from_entity(e: Self, position: &str) -> Self {
        let i = e.image.split("/").filter(|&x| x.ends_with(".png"))
            .map(|x| x.strip_suffix(".png"))
            .filter(|&x| x.is_some())
            .map(|x| x.unwrap()).collect::<Vec<_>>();
        let mut im = String::new();
        if i.len() > 0 {
            im = check_player_sprite(i[0]);
        } else {
            im = e.image;
        }
        let potion = Item::get_by_id("potionHP").unwrap();
        let bag = Bag::new(Some((potion, 5)), None, None, None);
        Self {
            display: e.display,
            image: im,
            character: e.character,
            attacks: e.attacks,
            position: position.to_string(),
            health: e.health,
            maxhealth: e.maxhealth,
            bag: Some(bag),
            status: None,
            lvl: 0,
            score: 0,
            room: "0".to_string(),
            floor: Some(0),
        }
    }

    pub fn new(display: char, image: Option<&str>, character: &str, attacks: Vec<String>, health: usize, lvl: usize) -> Self {
        let mut im = String::new();
        match image {
            Some(s) => im = format!("{}/entity/{}.png", conf::assets_path().unwrap(), s),
            _ => im = format!("{}/entity/horse.png", conf::assets_path().unwrap())
        };
        Self {
            display,
            image: im,
            character: character.to_string(),
            attacks,
            position: "None".to_string(),
            health,
            maxhealth: health,
            bag: None,
            status: None,
            lvl,
            score: 0,
            room: "None".to_string(),
            floor: None,
        }
    }

    pub fn players_list() -> Vec<Self> {
        let one = Self::new('1', Some("one"), "One", vec!("bite".to_string(), "divide".to_string(), "".to_string(), "".to_string()), 100, 1);
        let two = Self::new('2', Some("two"), "Two Snake", vec!("bite".to_string(), "venom".to_string(), "".to_string(), "".to_string()), 75, 1);
        let three = Self::new('3', Some("three"), "Three", vec!("bite".to_string(),"kick".to_string(), "".to_string(),"".to_string()), 75, 1);
        return vec!(one, two, three)
    }

    pub fn enemy_list() -> Vec<Self> {
        let pl = game::get_player();
        let rock = Self::new('Q', Some("rock"), "Rock", vec!("standStill".to_string(),"".to_string(), "".to_string(),"".to_string()), 5, 0);
        let horse = Self::new('h', Some("horse"), "El Horse", vec!("kick".to_string(), "standStill".to_string(), "standStill".to_string(),"".to_string()), 100, 1);
        let duck = Self::new('D', Some("duck"), "Quark", vec!("quack".to_string(), "i".to_string(), "quack".to_string(),"".to_string()), 100, 1);
        let mut e = vec!(rock, horse, duck);
        let p = Self::players_list().into_iter().filter(|x| *x.character != pl.character.clone()).collect::<Vec<Self>>();
        e.extend(p);
        return e
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
        let image = s.get("image").unwrap().to_string();
        let im = format!("{}/{}", conf::assets_path().unwrap(), image).to_string();
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
        let fldata = s.get("floor").unwrap().to_string();
        let mut floor: Option<usize> = None;
        if fldata != "None" {
            floor = Some(fldata.parse::<usize>().unwrap());
        }
        Self {
            display,
            image: im,
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
            floor,
        }
    }

    pub fn to_string(self) -> String {
        let mut st = "[player]\n".to_string();
        st += &format!("character = {}\n", self.character);
        let imag = self.image.clone();
        let imagi = imag.split("/").collect::<Vec<_>>();
        let ima = imagi[imagi.len()-1];
        st += &format!("image = {}",ima);
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
        match self.floor {
            Some(f) => st += &format!("floor = {}\n", f),
            _ => st += &format!("floor = None\n"),
        };
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
    pub fn move_to(&mut self, position: &str) {
        self.position = position.to_string();
    }
    pub fn move_room(&mut self, room: &str) {
        self.room = room.to_string();
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
    pub fn change_floor(&mut self, floor: usize) {
        self.floor = Some(floor);
    }
    pub fn add_attack(&mut self, attack: &str) {
        let att = self.attacks.clone();
        let atlist = Attack::list().into_iter().map(|x| x.id.to_string()).collect::<Vec<String>>();
        if atlist.contains(&attack.to_string()) {
            if att.contains(&attack.to_string()) {
                println!("posiadasz attack {}", attack);
            } else {
                if att.contains(&"".to_string()) {
                    let pos = att.iter().position(|r| *r == "".to_string()).unwrap();
                    self.attacks[pos] = attack.to_string();
                    println!("Posiadasz pusty slot {}", pos);
                } else {
                    println!("Nie posiadasz slotu");
                }
            }
        } else {
            println!("{} is not an attack", attack);
        }
    }
}
