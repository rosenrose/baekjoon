use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [n, m] = [(); 2].map(|_| input.next().unwrap());
    let mut baskets: Vec<_> = (1..=n).collect();

    for [mut i, mut j] in (0..m).map(|_| [(); 2].map(|_| input.next().unwrap() - 1)) {
        while i < j {
            baskets.swap(i, j);
            i += 1;
            j -= 1;
        }
    }

    for b in baskets {
        print!("{b} ");
    }
}
