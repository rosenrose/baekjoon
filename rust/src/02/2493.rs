use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let mut towers = Vec::with_capacity(500_000);

    for height in input.skip(1) {
        if towers.is_empty() {
            towers.push((height, 0));
            continue;
        }

        let (last_height, mut last_recv) = *towers.last().unwrap();

        if last_height > height {
            towers.push((height, towers.len() as i32));
            continue;
        }

        if last_recv == 0 {
            towers.push((height, 0));
            continue;
        }

        let (mut heighest, mut heighest_recv) = towers[last_recv as usize - 1];

        while heighest_recv != 0 && heighest < height {
            last_recv = heighest_recv;
            (heighest, heighest_recv) = towers[heighest_recv as usize - 1];
        }

        let receive = if heighest < height { 0 } else { last_recv };

        towers.push((height, receive));
    }

    for (_, i) in towers {
        write!(output, "{i} ").unwrap();
    }

    print!("{output}");
}
