fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();

    'outer: for num in (4..=n).rev() {
        let mut i = num;

        while i > 0 {
            if i % 10 != 4 && i % 10 != 7 {
                continue 'outer;
            }

            i /= 10;
        }

        println!("{num}");
        break;
    }
}
