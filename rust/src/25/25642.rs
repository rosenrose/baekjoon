enum Turn {
    Yt,
    Yj,
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [mut a, mut b] = parse_int_vec(&buf)[..] {
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
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
