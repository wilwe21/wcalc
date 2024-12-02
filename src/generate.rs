use rand::Rng;
use rand::seq::SliceRandom;
use std::collections::HashMap;

use crate::legend;
use crate::tentity::Entity;

pub fn generate_map(size: usize) -> String {
    let mut ran = rand::thread_rng();
    let mut map = String::new();
    for i in 0..(size) {
        map += &format!("{:#^size$}\n","#").to_string();
    }
    let mut map2 = map.split("\n").filter(|&x| x != "")
        .map(|x| x.split("").filter(|&xx| xx != "").map(|xxx| xxx.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();
    map2[0][0] = "0".to_string();
    let mut cur = vec![0,0];
    if size-1 <= 2 {
        return map2.into_iter().map(|x| x.join("")).collect::<Vec<String>>().join("\n")
    } else {
        for i in 1..10 {
            let num = ran.gen_range(0..2);
            if num == 0 && !(cur[0]+1 > size-1) {
                if cur[1] != 0 && cur[0] != 0 {
                    let ar = ran.gen_range(0..2);
                    if ar == 0 {
                        if map2[cur[0]-1][cur[1]-1] == "#" && map2[cur[0]][cur[1]-1] == "#" {
                            map2[cur[0]][cur[1]-1] = "a".to_string();
                        } 
                    } else if ar == 1 {
                        if map2[cur[0]-1][cur[1]] == "#" && map2[cur[0]-1][cur[1]-1] == "#" {
                            map2[cur[0]-1][cur[1]] = "a".to_string();
                        } 
                    }
                }
                cur[0] += 1;
                map2[cur[0]][cur[1]] = format!("{}",i);
            } else if !(cur[1]+1 > size-1)  {
                if cur[1] != 0 && cur[0] != 0 {
                    let ar = ran.gen_range(0..3);
                    if ar == 0 {
                        if map2[cur[0]][cur[1]-1] == "#" {
                            map2[cur[0]][cur[1]-1] = "a".to_string();
                        } 
                    } else if ar == 1 {
                        if map2[cur[0]-1][cur[1]] == "#" {
                            map2[cur[0]-1][cur[1]] = "a".to_string();
                        } 
                    }
                }
                cur[1] += 1;
                map2[cur[0]][cur[1]] = format!("{}",i);
            }
        }
    }
    return map2.into_iter().map(|x| x.join("")).collect::<Vec<String>>().join("\n")
}

pub fn add_enemys(room: &str) -> String {
    let enemys = Entity::enemy_list().into_iter().map(|r| r.display.to_string()).collect::<Vec<String>>();
    let ro = room.split("\n").filter(|r| *r != "").collect::<Vec<_>>();
    let midy = (ro.len()-1)/2;
    let midx = (ro[0].len()-1)/2;
    let mx = vec!((midy,midx),(1, midx), (ro.len()-2, midx), (midy, 1), (midy, ro[0].len()-2));
    let mut r2 = format!("{}\n", ro[0]).to_string();
    for i in 1..(ro.len()-1) {
        let mut row = ro[i].split("").filter(|r| *r != "").map(|r| r.to_string()).collect::<Vec<String>>();
        let mut encount = 0;
        for j in 1..(row.len()-1) {
            if let Some(_) = mx.iter().find(|&(x,y)| *x == i && *y == j) {
            } else {
                if encount > 3 {
                    break;
                }
                let ene = enemys.clone();
                let encho = ene.choose(&mut rand::thread_rng()).unwrap().clone();
                if rand::thread_rng().gen::<bool>() {
                    if rand::thread_rng().gen::<bool>() {
                        encount += 1;
                        row[j] = encho.to_string();
                    }
                }
            }
        }
        r2 += &format!("{}\n",row.join(""));
    }
    r2 += &format!("{}\n", ro[ro.len()-1]);
    r2.to_string()
}

pub fn generate_room(size: usize, dors: String, room_id: String) -> String {
    let mut room = String::new();
    let wall = legend::wall.clone();
    let door = legend::door.clone();
    let floor = legend::floor.clone();
    for i in 0..(size) {
        if i > 0 && i < (size-1) {
            let s2: usize = size - 2;
            if i == size / 2 {
                let mut left = String::new();
                let mut right = String::new();
                if dors.chars().collect::<Vec<_>>()[3] == '1' {
                    left = door.clone().to_string()
                } else {
                    left = wall.clone().to_string()
                }
                if dors.chars().collect::<Vec<_>>()[2] == '1'{
                    right = door.clone().to_string()
                } else {
                    right = wall.clone().to_string()
                }
                room += &format!("{}{}{}\n", left,floor.clone().to_string().repeat(s2),right).to_string();
                continue
            }
            room += &format!("{}{}{}\n",wall.clone(),floor.clone().to_string().repeat(s2),wall.clone()).to_string();
        } else {
            let w = (size - 1) / 2;
            if i == 0 && dors.chars().collect::<Vec<_>>()[0] == '1' {
                room += &format!("{}{}{}\n", wall.clone().to_string().repeat(w),door.clone(),wall.clone().to_string().repeat(w)).to_string();
                continue
            } 
            if i == (size-1) && dors.chars().collect::<Vec<_>>()[1] == '1' {
                room += &format!("{}{}{}\n", wall.clone().to_string().repeat(w),door.clone(),wall.clone().to_string().repeat(w)).to_string();
                continue
            }
            room += &format!("{}\n",wall.clone().to_string().repeat(size)).to_string();
        }
    }
    add_enemys(&room)
}

pub fn check_doors(map: HashMap<String, String>, y: usize, x: usize) -> String{
    // 0011 left right
    // 1100 down  up
    let mut vec = map.into_iter().collect::<Vec<_>>();
    vec.sort_by(|x,y| x.0.cmp(&y.0));
    let lines = vec.into_iter().map(|r| r.1).collect::<Vec<_>>();
    let vecvec = lines.iter()
        .map(|z| z.split("").filter(|&r| r != "").collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let checkx = |vecv: Vec<Vec<&str>>, x: usize, y: usize| -> u8 {
        let mut dor: u8 = 0;
        if x != 0 && vecv[y][x-1] != "#" {
            dor += 1
        }
        if x != (vecv[0].len()-1) && vecv[y][x+1] != "#" {
            dor += 2
        }
        return dor
    };
    let checky = |vecv: Vec<Vec<&str>>, x: usize, y: usize| -> u8{
        let mut dor: u8 = 0;
        if y != (vecv.len()-1) && vecvec[y+1][x] != "#" {
            dor += 4
        }
        if y != 0 && vecvec[y-1][x] != "#" {
            dor += 8
        }
        return dor
    };
    let lr = checkx(vecvec.clone(), y.clone(), x.clone());
    let ud = checky(vecvec.clone(), y.clone(), x.clone());
    return format!("{:<04b}", (lr+ud)).to_string()
}

pub fn generate_rooms(map: HashMap<String, String>, width: usize) -> HashMap<String, HashMap<String, String>>{
    let mut acount = 0;
    let mut rooms: HashMap<String, HashMap<String, String>> = HashMap::new();
    for i in 1..(map.len()+1) {
        let line = map.get(&i.to_string()).unwrap().clone();
        let livec: Vec<_> = line.split("").filter(|&x| x != "").collect::<Vec<_>>();
        let mut llc: Vec<_> = livec.clone();
        for z in &livec {
            let ind = llc.iter().position(|&r| r == z.clone()).unwrap();
            if *z == "a" {
                acount += 1;
                llc[ind] = "h";
                let mut r: HashMap<String,String> = HashMap::new();
                let dr = check_doors(map.clone(), (ind).clone(), (i-1).clone());
                let room = generate_room(width, dr,format!("a{}",acount).to_string());
                for (c,d) in room.split("\n").enumerate() {
                    if d != "" {
                        r.insert(c.to_string(), d.to_string());
                    }
                }
                rooms.insert(format!("rooma{}",acount).to_string(), r);
            } else if *z != "#"{
                let mut r: HashMap<String,String> = HashMap::new();
                let dr = check_doors(map.clone(), (ind).clone(), (i-1).clone());
                let room = generate_room(width, dr, z.to_string());
                for (c,d) in room.split("\n").enumerate() {
                    if d != "" {
                        r.insert(c.to_string(), d.to_string());
                    }
                }
                rooms.insert(format!("room{}",z).to_string(), r);
            }
        }
    }
    return rooms
}
