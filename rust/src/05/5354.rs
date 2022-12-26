use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf.lines().map(|s| s.parse::<usize>().unwrap());

    for n in input.skip(1) {
        if n == 1 {
            println!("#\n");
            continue;
        }

        println!("{}", "#".repeat(n));

        for _ in 0..n - 2 {
            println!("#{}#", "J".repeat(n - 2));
        }

        println!("{}\n", "#".repeat(n));
    }
}
