#[derive(Default, Copy, Clone, Debug)]
enum DNA {
    #[default]
    A,
    G,
    C,
    T,
}

use std::io;
use DNA::{A, C, G, T};

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut dna: Vec<_> = buf
        .lines()
        .next_back()
        .unwrap()
        .chars()
        .map(|ch| match ch {
            'A' => A,
            'G' => G,
            'C' => C,
            'T' => T,
            _ => Default::default(),
        })
        .collect();

    const TABLE: [[DNA; 4]; 4] = [[A, C, A, G], [C, G, T, A], [A, T, C, G], [G, A, G, T]];

    while dna.len() > 1 {
        let (col, row) = (dna.pop().unwrap(), dna.pop().unwrap());
        dna.push(TABLE[row as usize][col as usize]);
    }

    println!("{:?}", dna[0]);
}