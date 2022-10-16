fn main() {
    const N: usize = 9;
    let mut arr: [i32; N] = [0; N];

    let mut buf = String::new();

    for i in 0..N {
        read_line(&mut buf);
        arr[i] = parse_int(&buf);
    }

    let max = arr.iter().max().unwrap();
    let index = arr.iter().position(|n| n == max).unwrap();

    println!("{max}\n{}", index + 1);
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}
