use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let [n, m] = [(); 2].map(|_| input.next().unwrap());
    let count_2 =
        get_multiple_count(n, 2) - get_multiple_count(n - m, 2) - get_multiple_count(m, 2);
    let count_5 =
        get_multiple_count(n, 5) - get_multiple_count(n - m, 5) - get_multiple_count(m, 5);

    println!("{}", count_2.min(count_5));
}

fn get_multiple_count(num: i64, divisor: i64) -> i64 {
    let (mut count, mut pow) = (0, divisor);

    while pow <= num {
        count += num / pow;
        pow *= divisor;
    }

    count
}
