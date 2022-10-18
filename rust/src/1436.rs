fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();

    let mut num = 666;
    let mut count = 0;

    loop {
        let chars: Vec<char> = num.to_string().chars().collect();

        for i in 0..chars.len() - 2 {
            if (chars[i], chars[i + 1], chars[i + 2]) == ('6', '6', '6') {
                count += 1;
                break;
            }
        }

        if count == n {
            break;
        }

        num += 1;
    }

    println!("{num}");
}
