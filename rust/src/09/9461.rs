use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    for n in input.skip(1) {
        if n <= 3 {
            println!("1");
            continue;
        }
        if n == 4 {
            println!("2");
            continue;
        }

        let (mut a, mut b, mut c, mut d, mut e) = (1_i64, 1, 1, 2, 2);

        for _ in 5..n {
            (a, b, c, d, e) = (b, c, d, e, a + e);
        }

        println!("{e}");
    }
}
