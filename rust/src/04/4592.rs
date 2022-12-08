use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    loop {
        let n = input.next().unwrap();

        if n == 0 {
            return;
        }

        let mut arr: Vec<_> = (0..n).map(|_| input.next().unwrap()).collect();
        arr.dedup();

        for num in arr {
            print!("{num} ");
        }
        println!("$");
    }
}
