use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf.lines().map(|s| s.parse::<usize>().unwrap());

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
