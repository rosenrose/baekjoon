use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let (a, b, c) = (input(), input(), input());
    let mut car_count = [0; 100];

    for (on, off) in (0..3).map(|_| (input() as usize - 1, input() as usize - 1)) {
        for t in on..off {
            car_count[t] += 1;
        }
    }

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
