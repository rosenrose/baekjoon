use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let mut cards: [usize; 20] = std::array::from_fn(|i| i + 1);

    for [mut a, mut b] in (0..10).map(|_| [(); 2].map(|_| input.next().unwrap() - 1)) {
        while a < b {
            cards.swap(a, b);
            a += 1;
            b -= 1;
        }
    }

    for card in cards {
        print!("{card} ");
    }
}
