use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().map(parse_time);

    for (start, end) in (0..3).map(|_| (input.next().unwrap(), input.next().unwrap())) {
        let [mut h, mut m, mut s] = start[..] else { return };
        let mut count = 0;

        while [h, m, s] != end[..] {
            if (h + m + s) % 3 == 0 {
                count += 1;
            }
            // println!("{h}:{m}:{s}");
            s += 1;

            m += s / 60;
            s %= 60;

            h += m / 60;
            m %= 60;

            h %= 24;
        }

        if (h + m + s) % 3 == 0 {
            count += 1;
        }

        println!("{count}");
    }
}

fn parse_time(s: &str) -> Vec<i32> {
    s.split(':').flat_map(str::parse).collect()
}
