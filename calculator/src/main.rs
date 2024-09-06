use std::io;

fn getv() -> [String; 3] {
    println!("Enter value 1:");
    let mut v1 = String::new();
    io::stdin()
        .read_line(&mut v1)
        .expect("fail to read");
    println!("what to do [+-*/^√]:");
    let mut op = String::new();
    io::stdin()
        .read_line(&mut op)
        .expect("fail to read");
    println!("Enter value 2:");
    let mut v2 = String::new();
    io::stdin()
        .read_line(&mut v2)
        .expect("fail to read");
    v1 = v1.replace("\n", "");
    v2 = v2.replace("\n", "");
    op = op.replace("\n", "");
    [v1, op, v2]
}

fn domath(op: String, o1: u32, o2: u32) {
    if &op == "+" {
        let wyn = o1 + o2;
        println!("{}+{}={}", o1, o2, wyn);
    } else if &op == "*" {
        let wyn = o1 * o2;
        println!("{}*{}={}", o1, o2, wyn);
    } else if &op == "-" {
        let wyn = o1 - o2;
        println!("{}-{}={}", o1, o2, wyn);
    } else if &op == "/" {
        let wyn = o1 / o2;
        println!("{}/{}={}", o1, o2, wyn);
    } else if &op == "^" {
        let wyn = o1.pow(o2);
        println!("{}^{}={}", o1, o2, wyn);
    } else if &op == "√" {
        let f1 = o1 as f32;
        let f2 = o2 as f32;
        let wyn = f1.powf(1.0 / f2);
        println!("{}√{}={}", o1, o2, wyn);
    }

}

fn main() {
    loop {
        let vals = getv();
        domath(vals[1].clone(), vals[0].parse().expect("NoN"), vals[2].parse().expect("NoN")); 
    }
}
