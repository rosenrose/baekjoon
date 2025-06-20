use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [x, y] = [(); 2].map(|_| input.next().unwrap());
    let mut days = y;

    for i in 1..x {
        days += match i {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => 28,
            _ => unreachable!(),
        };
    }

    let day = match (days - 1) % 7 {
        0 => "MON",
        1 => "TUE",
        2 => "WED",
        3 => "THU",
        4 => "FRI",
        5 => "SAT",
        6 => "SUN",
        _ => unreachable!(),
    };

    println!("{day}");
}
