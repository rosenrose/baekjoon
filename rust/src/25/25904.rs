use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [n, x] = [(); 2].map(|_| input.next().unwrap());
    let (first, _) = input
        .enumerate()
        .min_by_key(|&(i, limit)| {
            let mut shout = x + i as i32;

            while shout <= limit {
                shout += n;
            }

            shout
        })
        .unwrap();

    println!("{}", first + 1);
}
