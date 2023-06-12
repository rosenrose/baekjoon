use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    while let [a, b, c, d] = [(); 4].map(|_| input.next().unwrap()) {
        if [a, b, c, d] == [0; 4] {
            return;
        }

        let ages = [c - a, c - b, d - a, d - b];

        println!(
            "{} {}",
            ages.iter().min().unwrap(),
            ages.iter().max().unwrap()
        );
    }
}
