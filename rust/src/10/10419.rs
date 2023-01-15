use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<i32>);

    for d in input.skip(1) {
        let mut t = 0;
        let t = loop {
            if t * t + t > d {
                break t - 1;
            }

            t += 1;
        };

        println!("{t}");
    }
}
