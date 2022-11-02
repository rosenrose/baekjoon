fn main() {
    const N: usize = 9;
    let mut arr: [i32; N] = [0; N];

    let mut buf = String::new();

    for i in 0..N {
        read_line(&mut buf);
        arr[i] = parse_int(&buf);
    }

    let (index, max) = arr.iter().enumerate().max_by_key(|(_, &n)| n).unwrap();
    let index = index + 1;

    println!("{max}\n{index}");
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}
