use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    input.next();

    let mut min = input.next().unwrap();
    let mut max = min;
    let mut max_height = 0;

    for height in input {
        if height > max {
            max = height;
            max_height = max_height.max(max - min);
        } else {
            (min, max) = (height, height);
        }
    }

    println!("{max_height}");
}
