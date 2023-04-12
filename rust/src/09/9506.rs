use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<i32>);

    for num in input.take_while(|&num| num != -1) {
        let mut divisors = Vec::new();

        for i in (1..).take_while(|i| i * i <= num) {
            if num % i != 0 {
                continue;
            }

            divisors.push(i);

            if i != num / i {
                divisors.push(num / i);
            }
        }

        divisors.sort();
        divisors.pop();

        let sum: i32 = divisors.iter().sum();

        if sum != num {
            println!("{num} is NOT perfect.");
            continue;
        }

        println!(
            "{num} = {}",
            format!("{divisors:?}")
                .replace(['[', ']'], "")
                .replace(", ", " + ")
        );
    }
}
