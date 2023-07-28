use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [a, b, c, d] = [(); 4].map(|_| input.next().unwrap());
    let [p, m, n] = [(); 3].map(|_| input.next().unwrap());

    let dog_atk_count = |time: i32, active: i32, rest: i32| (time % (active + rest) < active) as u8;

    for time in [p - 1, m - 1, n - 1] {
        println!("{}", dog_atk_count(time, a, b) + dog_atk_count(time, c, d));
    }
}
