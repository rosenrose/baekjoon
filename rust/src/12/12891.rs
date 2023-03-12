use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    let (_, p) = (input(), parse_int(input()));
    let dna = input().as_bytes();
    let mut min_counts = [0; 4];

    for i in 0..4 {
        min_counts[i] = parse_int(input());
    }

    let mut current_counts = [0_usize; 4];
    let mut pw_count = 0;
    let get_index = |ch: u8| match ch as char {
        'A' => 0,
        'C' => 1,
        'G' => 2,
        'T' => 3,
        _ => Default::default(),
    };

    for (i, window) in dna.windows(p).enumerate() {
        if i == 0 {
            for &ch in window {
                current_counts[get_index(ch)] += 1;
            }
        } else {
            current_counts[get_index(window[p - 1])] += 1;
        }

        if current_counts
            .iter()
            .zip(min_counts.iter())
            .all(|(cur, min)| cur >= min)
        {
            pw_count += 1;
        }

        current_counts[get_index(window[0])] -= 1;
    }

    println!("{pw_count}");
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
