use std::io;

const MAX: usize = 3;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().map(parse_time);

    let [current, mut end] = [(); 2].map(|_| input.next().unwrap());

    if end < current {
        end[0] += 24;
    }

    let mut diff = [0; MAX];

    for (i, num) in end.iter().zip(current).map(|(e, c)| e - c).enumerate() {
        diff[i] = num;
    }

    let [mut left_hour, mut left_minute, mut left_second] = diff;

    if left_second < 0 {
        left_second += 60;
        left_minute -= 1;
    }

    if left_minute < 0 {
        left_minute += 60;
        left_hour -= 1;
    }

    println!("{left_hour:02}:{left_minute:02}:{left_second:02}");
}

fn parse_time(s: &str) -> [i32; MAX] {
    let mut it = s.split(':').flat_map(str::parse);

    [(); 3].map(|_| it.next().unwrap())
}
