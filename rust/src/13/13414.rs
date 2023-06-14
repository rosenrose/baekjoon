use std::collections::HashMap;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let (k, n) = (input.next().unwrap(), input.next().unwrap() as usize);
    let mut clicks = HashMap::with_capacity(n >> 2);
    let waiting: Vec<_> = input
        .map(|num| {
            clicks.entry(num).and_modify(|c| *c += 1).or_insert(1);
            num
        })
        .collect();

    let mut count = 0;

    for num in waiting {
        clicks.entry(num).and_modify(|click| {
            *click -= 1;

            if *click == 0 {
                count += 1;
                writeln!(output, "{num:08}").unwrap();
            }
        });

        if count == k {
            break;
        }
    }

    print!("{output}");
}
