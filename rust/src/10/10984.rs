use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<f64>);
    let mut input = || input.next().unwrap();

    for _ in 0..input() as i32 {
        let n = input() as i32;
        let mut credits = 0;

        let sum: f64 = (0..n)
            .map(|_| {
                let (c, g) = (input() as i32, input());

                credits += c;
                c as f64 * g
            })
            .sum();

        let gpa = sum / credits as f64;

        println!("{credits} {gpa:.1}");
    }
}
