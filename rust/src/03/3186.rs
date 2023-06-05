use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut output = String::new();

    let using = "1".repeat(parse_int(input.next().unwrap()));
    let finished = "0".repeat(parse_int(input.next().unwrap()));
    let data = input.skip(1).next().unwrap();

    let mut cursor = 0;

    while let Some(mut start) = data[cursor..].find(&using) {
        start += cursor;

        let flush_time = if let Some(mut end) = data[start..].find(&finished) {
            end += start;
            end + finished.len()
        } else {
            data.trim_end_matches('0').len() + finished.len()
        };

        writeln!(output, "{flush_time}").unwrap();
        cursor = flush_time.min(data.len());
    }

    if output.is_empty() {
        println!("NIKAD");
        return;
    }

    print!("{output}");
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
