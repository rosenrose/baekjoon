use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    #[rustfmt::skip]
    let values: Vec<_> = buf
        .lines()
        .enumerate()
        .map(|(i, s)| match s {
            "black"  => if i < 2 { 0 } else { 1 },
            "brown"  => if i < 2 { 1 } else { 10 },
            "red"    => if i < 2 { 2 } else { 100 },
            "orange" => if i < 2 { 3 } else { 1_000 },
            "yellow" => if i < 2 { 4 } else { 10_000 },
            "green"  => if i < 2 { 5 } else { 100_000 },
            "blue"   => if i < 2 { 6 } else { 1_000_000 },
            "violet" => if i < 2 { 7 } else { 10_000_000 },
            "grey"   => if i < 2 { 8 } else { 100_000_000 },
            "white"  => if i < 2 { 9 } else { 1_000_000_000 },
            _ => Default::default(),
        })
        .collect();

    println!("{}", (values[0] * 10 + values[1]) * values[2]);
}
