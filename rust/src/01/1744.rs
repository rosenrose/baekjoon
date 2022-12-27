use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf.lines().map(|s| s.parse::<i32>().unwrap());

    let mut nums: Vec<_> = input.skip(1).collect();
    nums.sort();

    let (mut sum, mut product_sum) = (0, 0);

    while let Some(&n) = nums.get(1) {
        if n > 0 {
            break;
        }

        product_sum += nums.remove(0) * nums.remove(0);
    }

    while let Some(&n) = nums.iter().nth_back(1) {
        if n <= 0 {
            break;
        }

        let (a, b) = (nums.pop().unwrap(), nums.pop().unwrap());

        if a == 1 || b == 1 {
            sum += a + b;
        } else {
            product_sum += a * b;
        }
    }

    sum += nums.iter().sum::<i32>();

    println!("{}", sum + product_sum);
}
