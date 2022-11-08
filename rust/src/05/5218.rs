fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();

    for _ in 0..n {
        read_line(&mut buf);

        if let [a, b] = parse_str_vec(&buf)[..] {
            let distances = a.as_bytes().iter().zip(b.as_bytes()).map(|(x, y)| {
                if x <= y {
                    y - x
                } else {
                    26 - (x - y)
                }
            });

            print!("Distances: ");
            for dist in distances {
                print!("{dist} ");
            }

            println!("");
        }
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_str_vec(buf: &String) -> Vec<&str> {
    buf.split_whitespace().collect()
}
