use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    const N: usize = 3;
    let (mut x_points, mut y_points) = ([0; N], [0; N]);

    for i in 0..N {
        (x_points[i], y_points[i]) = (input.next().unwrap(), input.next().unwrap());
    }

    let get_unique = |points: [i32; 3]| match points {
        [a, b, c] if a == b => c,
        [a, b, c] if a == c => b,
        [a, b, c] if b == c => a,
        _ => unreachable!(),
    };

    println!("{} {}", get_unique(x_points), get_unique(y_points));
}
