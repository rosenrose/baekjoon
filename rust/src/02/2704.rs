use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().map(parse_time);
    let mut output = String::new();

    for bin_time in input.skip(1) {
        let vertical = (0..bin_time[0].len())
            .map(|i| ((0..3).flat_map(|j| bin_time[j].chars().nth(i))).collect())
            .collect::<Vec<String>>()
            .join("");
        let horizontal = bin_time.join("");

        writeln!(output, "{vertical} {horizontal}").unwrap();
    }

    print!("{output}");
}

fn parse_time(s: &str) -> Vec<String> {
    s.split(':')
        .map(|s| format!("{:06b}", s.parse::<i32>().unwrap()))
        .collect()
}
