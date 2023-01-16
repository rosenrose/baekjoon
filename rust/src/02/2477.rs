use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let k = input();

    let mut longest = 0;
    let mut lengths: Vec<_> = (0..6)
        .map(|_| {
            let (_, length) = (input(), input());
            longest = length.max(longest);

            length
        })
        .collect();

    while lengths[0] != longest {
        lengths.rotate_left(1);
    }

    let (outer_area, inner_area) = if lengths[1] < lengths[5] {
        (lengths[0] * lengths[5], lengths[2] * lengths[3])
    } else {
        (lengths[0] * lengths[1], lengths[3] * lengths[4])
    };

    println!("{}", (outer_area - inner_area) * k);
}
