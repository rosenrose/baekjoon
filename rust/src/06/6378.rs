use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace();

    for num in input.take_while(|&num| num != "0") {
        let mut sum: u32 = num.chars().map(|c| c.to_digit(10).unwrap()).sum();

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
