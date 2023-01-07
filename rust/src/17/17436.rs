use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap());

    let (n, m) = (input.next().unwrap(), input.next().unwrap());
    let nums: Vec<_> = input.collect();
    let mut count = 0;

    for bit in 1..1 << n {
        let indices: Vec<_> = (0..n).filter(|index| bit & (1 << index) != 0).collect();
        let mut lcm = 1;

        for num in indices.iter().map(|&i| nums[i as usize]) {
            lcm *= num;

            if lcm > m {
                break;
            }
        }

        let multiple_count = m / lcm;

        count += if indices.len() % 2 == 1 {
            multiple_count
        } else {
            -multiple_count
        };
    }

    println!("{count}");
}
