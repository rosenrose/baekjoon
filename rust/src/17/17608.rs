use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<i32>);

    let mut bars: Vec<_> = input.skip(1).collect();
    let (mut max, mut count) = (bars.pop().unwrap(), 1);

    while let Some(height) = bars.pop() {
        if height > max {
            max = height;
            count += 1;
        }
    }

    println!("{count}");
}
