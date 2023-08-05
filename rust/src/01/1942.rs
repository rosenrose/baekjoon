use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().map(parse_time);

    for [mut start, end] in (0..3).map(|_| [(); 2].map(|_| input.next().unwrap())) {
        let mut count = 0;

        loop {
            if start.iter().sum::<i32>() % 3 == 0 {
                count += 1;
            }

            if start == end {
                break;
            }
            // println!("{start:?}");
            start[2] += 1;

            start[1] += start[2] / 60;
            start[2] %= 60;

            start[0] += start[1] / 60;
            start[1] %= 60;

            start[0] %= 24;
        }

        println!("{count}");
    }
}

fn parse_time(s: &str) -> Vec<i32> {
    s.split(':').flat_map(str::parse).collect()
}
