use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf.lines().map(|s| s.parse::<i32>().unwrap());

    if let [start, end, ice, melting, water] = input.collect::<Vec<_>>()[..] {
        let mut time = (end - start.max(0)) * water;

        if start < 0 {
            time += -start * ice;
            time += melting;
        }

        println!("{time}");
    }
}
