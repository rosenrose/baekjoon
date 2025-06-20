#[derive(Copy, Clone, Default, Debug)]
enum DNA {
    #[default]
    A,
    G,
    C,
    T,
}

use std::io;
use DNA::*;

const MAX: usize = 1_000_000;
const TABLE: [[DNA; 4]; 4] = [[A, C, A, G], [C, G, T, A], [A, T, C, G], [G, A, G, T]];

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let mut n: usize = input.next().unwrap().parse().unwrap();
    let mut dna = [Default::default(); MAX];

    for (i, ch) in input.next().unwrap().char_indices() {
        dna[i] = match ch {
            'A' => A,
            'G' => G,
            'C' => C,
            'T' => T,
            _ => unreachable!(),
        };
    }

    while n > 1 {
        let (col, row) = (dna[n - 1], dna[n - 2]);

        n -= 1;
        dna[n - 1] = TABLE[row as usize][col as usize];
    }

    println!("{:?}", dna[0]);
}
