use std::collections::HashSet;
use std::fmt::Write;
use std::io;

const MOD: usize = 650_011;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let n = input.next().unwrap() as usize;
    // let mut hash_table = [None; MOD];

    // for num in input.by_ref().take(n) {
    //     insert(num, &mut hash_table);
    // }

    // for num in input.skip(1) {
    //     write!(output, "{} ", has(num, &hash_table) as u8).unwrap();
    // }

    let mut cards = HashSet::with_capacity(n);

    for num in input.by_ref().take(n) {
        cards.insert(num);
    }

    for num in input.skip(1) {
        write!(output, "{} ", cards.contains(&num) as u8).unwrap();
    }

    print!("{output}");
}

fn insert(input: i32, hash_table: &mut [Option<i32>]) {
    let mut idx = input.rem_euclid(MOD as i32) as usize;

    while let Some(num) = hash_table[idx] {
        if num == input {
            return;
        }

        idx = (idx + 1) % MOD;
    }

    hash_table[idx] = Some(input);
}

fn has(input: i32, hash_table: &[Option<i32>]) -> bool {
    let mut idx = input.rem_euclid(MOD as i32) as usize;

    while let Some(num) = hash_table[idx] {
        if num == input {
            return true;
        }

        idx = (idx + 1) % MOD;
    }

    false
}
