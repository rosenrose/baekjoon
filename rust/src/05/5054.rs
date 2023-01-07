use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    for _ in 0..input.next().unwrap() {
        let n = input.next().unwrap();

        let mut stores: Vec<_> = (0..n).map(|_| input.next().unwrap()).collect();
        stores.sort();

        let min = stores[0];
        let max = *stores.last().unwrap();

        let min_distance = (min..=max)
            .map(|i| {
                i.abs_diff(min)
                    + i.abs_diff(max)
                    + (1..stores.len())
                        .map(|j| stores[j - 1].abs_diff(stores[j]))
                        .sum::<u32>()
            })
            .min()
            .unwrap();

        println!("{min_distance}");
    }
}
