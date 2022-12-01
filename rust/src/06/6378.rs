fn main() {
    let mut buf = String::new();

    loop {
        read_line(&mut buf);

        if buf.trim() == "0" {
            return;
        }

        let mut sum: u32 = buf.trim().chars().map(|c| c.to_digit(10).unwrap()).sum();

        if sum < 10 {
            println!("{sum}");
            continue;
        }

        let mut digital_root = 0;

        loop {
            while sum > 0 {
                digital_root += sum % 10;
                sum /= 10;
            }

            if digital_root < 10 {
                break;
            }

            (sum, digital_root) = (digital_root, 0);
        }

        println!("{digital_root}");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
