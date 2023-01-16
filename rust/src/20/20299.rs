use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (n, team, personal) = (input(), input(), input());
    let mut count = 0;

    (0..n)
        .filter_map(|_| {
            let rating = [input(), input(), input()];

            (rating.iter().sum::<i32>() >= team && rating.iter().all(|&r| r >= personal))
                .then_some(rating)
        })
        .for_each(|[x1, x2, x3]| {
            count += 1;
            write!(output, "{x1} {x2} {x3} ").unwrap();
        });

    println!("{count}\n{output}");
}
