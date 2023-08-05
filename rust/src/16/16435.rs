use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [_, init] = [(); 2].map(|_| input.next().unwrap());

    let mut heights: Vec<_> = input.collect();
    heights.sort();

    let length = heights.iter().fold(
        init,
        |acc, &height| if acc >= height { acc + 1 } else { acc },
    );

    println!("{length}");
}
