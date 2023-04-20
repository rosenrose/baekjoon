use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let mut sum = 0;
    let mut product_sum = 0;

    for (i, num) in input.skip(1).enumerate() {
        if i == 0 {
            sum = num;
            continue;
        }

        product_sum += num * sum;
        sum += num;
    }

    println!("{product_sum}");
}
