use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<usize>);

    const N: usize = 30;
    let mut is_submit = [false; N];

    for num in input {
        is_submit[num - 1] = true;
    }

    let absent = is_submit
        .iter()
        .enumerate()
        .filter_map(|(num, &submit)| (!submit).then(|| num + 1));

    for num in absent {
        println!("{num}");
    }
}
