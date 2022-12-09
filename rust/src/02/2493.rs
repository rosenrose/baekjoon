use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    let mut towers = Vec::new();

    for height in input.skip(1) {
        if towers.is_empty() {
            towers.push((height, 0));
            continue;
        }

        let (last_height, mut last_recv) = *towers.last().unwrap();

        if last_height > height {
            towers.push((height, towers.len()));
            continue;
        }

        if last_recv == 0 {
            towers.push((height, 0));
            continue;
        }

        let (mut heighest, mut heighest_recv) = towers[last_recv - 1];

        while heighest_recv != 0 && heighest < height {
            last_recv = heighest_recv;
            (heighest, heighest_recv) = towers[heighest_recv - 1];
        }

        let receive = if heighest < height { 0 } else { last_recv };

        towers.push((height, receive));
    }

    for (_, i) in towers {
        write!(output, "{i} ").unwrap();
    }

    print!("{output}");
}
