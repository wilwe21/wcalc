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
}
