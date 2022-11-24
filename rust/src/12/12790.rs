use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut lines = buf.lines();
    lines.next();

    lines.for_each(|line| {
        if let [hp, mp, atk, def, hp_delta, mp_delta, atk_delta, def_delta] =
            parse_int_vec(line)[..]
        {
            let power = (hp + hp_delta).max(1)
                + 5 * (mp + mp_delta).max(1)
                + 2 * (atk + atk_delta).max(0)
                + 2 * (def + def_delta);

            writeln!(stdout, "{power}").unwrap();
        }
    });
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
