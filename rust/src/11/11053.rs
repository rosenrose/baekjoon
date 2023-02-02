use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let _n = input.next();
    let mut lis_temp = vec![input.next().unwrap()];

    for num in input {
        if num > *lis_temp.last().unwrap() {
            lis_temp.push(num);
            continue;
        }

        let pos = lis_temp.binary_search(&num).unwrap_or_else(|i| i);
        lis_temp[pos] = num;
    }

    println!("{}", lis_temp.len());
}
