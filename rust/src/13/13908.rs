use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let (n, _) = (input.next().unwrap() as u32, input.next());
    let nums: Vec<_> = input.collect();

    let max = 10_usize.pow(n);
    let mut count = 0;

    for mut num in 0..max {
        let mut digits = [false; 10];

        if num < max / 10 {
            digits[0] = true;
        }

        while num > 0 {
            digits[num % 10] = true;
            num /= 10;
        }

        if nums.iter().all(|&num| digits[num]) {
            count += 1;
        }
    }

    println!("{count}");
}
