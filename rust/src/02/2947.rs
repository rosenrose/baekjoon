fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut arr = parse_int_vec(&buf);
    let len = arr.len();

    for i in 0..len - 1 {
        for j in 1..len - i {
            if arr[j - 1] <= arr[j] {
                continue;
            }

            arr.swap(j - 1, j);

            for num in &arr {
                print!("{num} ");
            }
            println!("");
        }
    }
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
