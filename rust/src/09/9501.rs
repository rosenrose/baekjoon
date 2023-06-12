use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    for _ in 0..input() {
        let (n, d) = (input(), input());
        let count = (0..n)
            .filter(|_| {
                let [v, f, c] = [(); 3].map(|_| input());
                v * f / c >= d
            })
            .count();

        println!("{count}");
    }
}
