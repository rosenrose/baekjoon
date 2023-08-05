use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [n, _] = [(); 2].map(|_| input.next().unwrap());
    let nums: Vec<_> = input.collect();

    let count = product(0, n, &mut [0; 10], &nums);

    println!("{count}");
}

fn product(depth: usize, n: usize, digits: &mut [i32], nums: &[usize]) -> i32 {
    if depth == n {
        return nums.iter().all(|&num| digits[num] > 0) as i32;
    }

    (0..10)
        .map(|digit| {
            digits[digit] += 1;
            let result = product(depth + 1, n, digits, nums);
            digits[digit] -= 1;

            result
        })
        .sum()
}
