use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<i32>);

    for n in input.skip(1) {
        match n {
            0..=1 => println!("1"),
            2 => println!("2"),
            3 => println!("4"),
            _ => {
                let (mut a, mut b, mut c, mut d) = (1_i64, 1, 2, 4);

                for _ in 0..n - 3 {
                    (a, b, c, d) = (b, c, d, a + b + c + d);
                }

                println!("{d}");
            }
        }
    }
}
