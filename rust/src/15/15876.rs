fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n, k] = parse_int_vec(&buf)[..] else { return };
    let k = k - 1;
    let max_len = k + n * 4;

    let mut bin = String::new();

    for i in 0.. {
        bin = [&bin[..], &format!("{i:b}")].concat();

        if bin.len() > max_len {
            break;
        }
    }
    // println!("{bin}");
    for i in 0..5 {
        print!("{} ", bin.chars().nth(k + n * i).unwrap());
    }
}

fn parse_int_vec(buf: &String) -> Vec<usize> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
