use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let (mut x, mut y) = (0_i32, 0);

    for winner in buf.lines().skip(1) {
        match winner {
            "D" => x += 1,
            "P" => y += 1,
            _ => unreachable!(),
        }

        if x.abs_diff(y) == 2 {
            break;
        }
    }

    println!("{x}:{y}");
}
