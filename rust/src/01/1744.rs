use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<i32>);

    let mut nums: Vec<_> = input.skip(1).collect();
    nums.sort();

    let (mut sum, mut product_sum) = (0, 0);

    while let Some(_n @ ..=0) = nums.get(1) {
        product_sum += nums.remove(0) * nums.remove(0);
    }

    while let Some(_n @ 1..) = nums.iter().nth_back(1) {
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
