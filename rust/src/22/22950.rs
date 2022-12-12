use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.lines();
    input.next();

    let m = input.next().unwrap();
    let k: usize = input.next().unwrap().parse().unwrap();

    if k == 0 || m.chars().all(|c| c == '0') {
        println!("YES");
        return;
    }

    if m.len() < k {
        println!("NO");
        return;
    }

    if m[m.len() - k..].chars().all(|c| c == '0') {
        println!("YES");
        return;
    }

    println!("NO");
}
