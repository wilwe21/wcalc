use std::io;

fn movein(st: &str) -> String {
    let s: Vec<_> = st.chars().collect();
    let mut new = vec![s[s.len()-1]];
    for i in 0..s.len()-1 {
        new.push(s[i])
    }
    let compleat = new.iter().map(|c| c.to_string()).collect::<Vec<String>>().join("").clone();
    compleat
}

fn compress(st: &str) -> String {
    let s: Vec<_> = st.chars().collect();
    let mut vec: Vec<Vec<String>> = Vec::new();
    let mut count = 0;
    let mut cur: char = '`'; 
    for a in 0..s.len() {
        if cur == '`' {
            cur = s[a];
            count += 1;
        } else if cur == s[a] {
            count += 1;
        } else {
            if count == 1 {
                vec.push(vec![cur.to_string()]);
            } else {
                vec.push(vec![count.to_string(), cur.to_string()]);
            }
            cur = s[a];
            count = 1;
        }
    }
    if count == 1 {
        vec.push(vec![cur.to_string()]);
    } else {
        vec.push(vec![count.to_string(), cur.to_string()]);
    }
    let mut end: Vec<String> = Vec::new();
    for i in vec {
        end.push(i.iter().map(|c| c.to_string()).collect::<Vec<String>>().join("").clone());
    }
    end.join("").clone()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = format!("{}$", input.replace("\n", ""));
    let mut inp: Vec<String> = vec![input.clone()];
    for i in 0..input.len()-1 {
        let st = movein(&inp[i]);
        inp.push(st);
    }
    inp.sort();
    let mut end: Vec<char> = Vec::new();
    for i in inp {
        let s: Vec<_> = i.chars().collect();
        end.push(s[s.len()-1]);
    }
    let fin = end.iter().map(|c| c.to_string()).collect::<Vec<String>>().join("").clone();
    println!("{}", fin);
    let compressed = compress(&fin);
    println!("{}", compressed);
}
