use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<u64>);

    let [c, b] = [(); 2].map(|_| input.next_back().unwrap());

    let watchers: u64 = input
        .skip(1)
        .map(|count| 1 + count.saturating_sub(b).div_ceil(c))
        .sum();

    println!("{watchers}");
}
