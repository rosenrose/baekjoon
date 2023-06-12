use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);
    let mut input = || input.next().unwrap();

    for _ in 0..input() {
        let m = input();
        let rewards: Vec<_> = (0..m).map(|_| [(); 3].map(|_| input())).collect();
        let [k, d, a] = [(); 3].map(|_| input());

        let donation: i64 = rewards
            .iter()
            .map(|reward| (k * reward[0] - d * reward[1] + a * reward[2]).max(0))
            .sum();

        println!("{donation}");
    }
}
