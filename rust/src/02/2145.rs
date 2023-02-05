use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<i32>);

    let digit_sum = |n: i32| n.to_string().chars().map(|c| c as i32 - '0' as i32).sum();

    for mut num in input.take_while(|&n| n != 0) {
        while num >= 10 {
            num = digit_sum(num);
        }

        println!("{num}");
    }
}
