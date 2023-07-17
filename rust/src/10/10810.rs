use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [n, m] = [(); 2].map(|_| input.next().unwrap());
    let mut balls = vec![0; n];

    for [i, j, k] in (0..m).map(|_| [(); 3].map(|_| input.next().unwrap())) {
        for idx in i - 1..=j - 1 {
            balls[idx] = k;
        }
    }

    for b in balls {
        print!("{b} ");
    }
}
