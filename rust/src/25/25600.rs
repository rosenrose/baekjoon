use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap();
    let max_score = (0..n)
        .map(|_| {
            let [a, d, g] = [(); 3].map(|_| input.next().unwrap());
            let score = a * (d + g);

            if a == d + g {
                score * 2
            } else {
                score
            }
        })
        .max()
        .unwrap();

    println!("{max_score}");
}
