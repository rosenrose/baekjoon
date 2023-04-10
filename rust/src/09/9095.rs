use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<i32>);

    for num in input.skip(1) {
        match num {
            1 | 2 => println!("{num}"),
            3 => println!("4"),
            _ => {
                let (mut a, mut b, mut c) = (1, 2, 4);

                for _ in 0..num - 3 {
                    (a, b, c) = (b, c, a + b + c);
                }

                println!("{c}");
            }
        }
    }
}
