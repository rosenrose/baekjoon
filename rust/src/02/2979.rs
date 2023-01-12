use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    let (a, b, c) = (input(), input(), input());
    let car_count = (0..3).fold([0; 100], |mut acc, _| {
        let (on, off) = (input() as usize - 1, input() as usize - 1);

        for t in on..off {
            acc[t] += 1;
        }

        acc
    });

    let cost: i32 = car_count
        .iter()
        .map(|count| match count {
            3 => c * 3,
            2 => b * 2,
            1 => a,
            _ => 0,
        })
        .sum();

    println!("{cost}");
}
