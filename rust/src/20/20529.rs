use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    const MBTI_COUNT: usize = 16;

    for _ in 0..parse_int(input.next().unwrap()) {
        let n = parse_int(input.next().unwrap());
        let mbti = input.by_ref().take(n);

        if n > MBTI_COUNT * 2 {
            mbti.for_each(|_| ());
            println!("0");
            continue;
        }

        let mbti: Vec<_> = mbti.map(str::as_bytes).collect();
        let mut min_dist = usize::MAX;

        for a in 0..n - 2 {
            for b in a + 1..n - 1 {
                for c in b + 1..n {
                    let ab = mbti[a].iter().zip(mbti[b]).filter(|(x, y)| x != y).count();
                    let bc = mbti[b].iter().zip(mbti[c]).filter(|(x, y)| x != y).count();
                    let ac = mbti[a].iter().zip(mbti[c]).filter(|(x, y)| x != y).count();

                    min_dist = min_dist.min(ab + bc + ac);
                }
            }
        }

        println!("{min_dist}");
    }
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
