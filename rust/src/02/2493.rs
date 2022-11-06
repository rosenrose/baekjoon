use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf.clear();
    stdin.read_line(&mut buf).unwrap();

    let mut towers = Vec::new();

    buf.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .for_each(|height| {
            if towers.is_empty() {
                towers.push((height, 0));
                return;
            }

            let (last_height, mut last_recv) = *towers.last().unwrap();

            if last_height > height {
                towers.push((height, towers.len()));
                return;
            }

            if last_recv == 0 {
                towers.push((height, 0));
                return;
            }

            let (mut heighest, mut heighest_recv) = towers[last_recv - 1];

            while heighest_recv != 0 && heighest < height {
                last_recv = heighest_recv;
                (heighest, heighest_recv) = towers[heighest_recv - 1];
            }

            let receive = if heighest < height { 0 } else { last_recv };

            towers.push((height, receive));
        });

    for (_, i) in towers {
        write!(stdout, "{i} ").unwrap();
    }
}
