fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n, k] = parse_int_vec(&buf)[..] else {
        return;
    };
    let k = k - 1;
    let max_len = k + n * 4;

    let mut bin = String::new();

    for i in 0.. {
        bin = format!("{bin}{i:b}");

        if bin.len() > max_len {
            break;
        }
    }
    // println!("{bin}");
    for ch in (0..5).flat_map(|i| bin.chars().nth(k + n * i)) {
        print!("{ch} ");
    }
}

fn parse_int_vec(buf: &str) -> Vec<usize> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
