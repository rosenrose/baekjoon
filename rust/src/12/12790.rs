use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let stdin = stdin();
    let mut stdin = stdin.lock();

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    for _ in 0..input.next().unwrap() {
        if let [hp, mp, atk, def, hp_delta, mp_delta, atk_delta, def_delta] =
            (0..8).map(|_| input.next().unwrap()).collect::<Vec<_>>()[..]
        {
            let power = (hp + hp_delta).max(1)
                + 5 * (mp + mp_delta).max(1)
                + 2 * (atk + atk_delta).max(0)
                + 2 * (def + def_delta);

            writeln!(output, "{power}").unwrap();
        }
    }

    print!("{output}");
}
