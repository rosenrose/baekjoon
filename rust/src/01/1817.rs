use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [_, max_weight] = [(); 2].map(|_| input.next().unwrap());
    let mut count = 0;
    let mut weight_sum = 0;

    for weight in input {
        weight_sum += weight;

        if weight_sum > max_weight {
            count += 1;
            weight_sum = weight;
        }
    }

    if weight_sum > 0 {
        count += 1;
    }

    println!("{count}");
}
