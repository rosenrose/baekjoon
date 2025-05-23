use std::fmt::Write;
use std::io;

const MAX: usize = 1000 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();
    let mut output = String::new();

    while let pages @ 1.. = parse_int(input.next().unwrap()) {
        let mut is_print = [false; MAX];

        for range in input.next().unwrap().split(',') {
            let mut range = range.split('-').map(parse_int);
            let low = range.next().unwrap();
            let high = range.next().unwrap_or(low);

            for i in low..=high.min(pages) {
                is_print[i] = true;
            }
        }

        writeln!(
            output,
            "{}",
            is_print[..=pages].iter().filter(|&&p| p).count()
        )
        .unwrap();
    }

    print!("{output}");
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
