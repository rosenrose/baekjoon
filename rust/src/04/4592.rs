use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    while let Some(n @ 1..) = input.next() {
        let mut arr: Vec<_> = input.by_ref().take(n).collect();
        arr.dedup();

        for num in arr {
            print!("{num} ");
        }
        println!("$");
    }
}
