fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let arr = parse_int_vec(&buf);

    if (1..arr.len()).all(|i| arr[i - 1] < arr[i]) {
        println!("ascending");
        return;
    }

    if (1..arr.len()).all(|i| arr[i - 1] > arr[i]) {
        println!("descending");
        return;
    }

    println!("mixed");
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
