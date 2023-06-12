use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    for _ in 0..input() {
        let [hp, mp, atk, def] = [(); 4].map(|_| input());
        let power = (hp + input()).max(1)
            + 5 * (mp + input()).max(1)
            + 2 * (atk + input()).max(0)
            + 2 * (def + input());

        writeln!(output, "{power}").unwrap();
    }

    print!("{output}");
}
