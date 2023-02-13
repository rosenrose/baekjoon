use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    'outer: for i in 1..=input() {
        let credit = input();
        let prices: Vec<_> = (0..input()).map(|_| input()).collect();

        for a in 0..prices.len() - 1 {
            for b in a + 1..prices.len() {
                if prices[a] + prices[b] == credit {
                    println!("Case #{i}: {} {}", a + 1, b + 1);
                    continue 'outer;
                }
            }
        }
    }
}
