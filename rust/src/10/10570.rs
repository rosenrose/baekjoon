use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    for _ in 0..input() {
        let mut max_count = 0;
        let mut counts = [0; 1001];

        for num in (0..input()).map(|_| input()) {
            counts[num] += 1;
            max_count = counts[num].max(max_count);
        }

        let (most, _) = counts
            .iter()
            .enumerate()
            .find(|(_, &c)| c == max_count)
            .unwrap();

        writeln!(output, "{most}").unwrap();
    }

    print!("{output}");
}
