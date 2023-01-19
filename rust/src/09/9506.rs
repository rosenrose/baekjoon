use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<i32>);

    for num in input.take_while(|&num| num != -1) {
        let mut divisors = (1..)
            .take_while(|i| i * i <= num)
            .fold(Vec::new(), |mut acc, i| {
                if num % i == 0 {
                    acc.push(i);
                    acc.push(num / i);
                }

                acc
            });

        divisors.dedup();
        divisors.sort();
        divisors.pop();

        let sum: i32 = divisors.iter().sum();

        if sum == num {
            println!(
                "{num} = {}",
                format!("{divisors:?}")
                    .replace(['[', ']'], "")
                    .replace(", ", " + ")
            );

            continue;
        }

        println!("{num} is NOT perfect.");
    }
}
