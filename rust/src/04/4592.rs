use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    while let Some(n @ 1..) = input.next() {
        let mut arr: Vec<_> = (0..n).map(|_| input.next().unwrap()).collect();
        arr.dedup();

        for num in arr {
            print!("{num} ");
        }
        println!("$");
    }
}
