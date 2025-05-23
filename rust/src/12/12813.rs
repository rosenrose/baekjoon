use std::io;

const MAX: usize = 100_000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();
    let [a, b] = [(); 2].map(|_| input.next().unwrap());

    let mut results = [['\0'; MAX]; 5];

    for (i, (a, b)) in a.chars().zip(b.chars()).enumerate() {
        results[0][i] = if a == '1' && b == '1' { '1' } else { '0' };
        results[1][i] = if a == '1' || b == '1' { '1' } else { '0' };
        results[2][i] = if a != b { '1' } else { '0' };
        results[3][i] = if a == '1' { '0' } else { '1' };
        results[4][i] = if b == '1' { '0' } else { '1' };
    }

    for result in results {
        println!("{}", String::from_iter(result));
    }
}
