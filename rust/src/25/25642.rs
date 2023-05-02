enum Turn {
    Yt,
    Yj,
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [mut a, mut b] = parse_int_vec(&buf)[..] else { return };
    let mut turn = Turn::Yt;

    while a < 5 && b < 5 {
        match turn {
            Turn::Yt => {
                b += a;
                turn = Turn::Yj
            }
            Turn::Yj => {
                a += b;
                turn = Turn::Yt
            }
        }
    }

    println!(
        "{}",
        match turn {
            Turn::Yt => "yj",
            Turn::Yj => "yt",
        }
    )
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
