use std::collections::HashMap;
use std::fmt::Write;
use std::io;

const MOD: usize = 650_011;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let n = input.next().unwrap() as usize;
    // let mut key_table = [None; MOD];
    // let mut value_table = [None; MOD];

    // for num in input.by_ref().take(n) {
    //     insert(num, &mut key_table, &mut value_table);
    // }

    // for num in input.skip(1) {
    //     write!(
    //         output,
    //         "{} ",
    //         get(num, &key_table, &value_table).unwrap_or(0)
    //     )
    //     .unwrap();
    // }

    let mut counts = HashMap::with_capacity(n >> 1);

    for num in input.by_ref().take(n) {
        counts.entry(num).and_modify(|c| *c += 1).or_insert(1);
    }

    for num in input.skip(1) {
        write!(output, "{} ", counts.get(&num).unwrap_or(&0)).unwrap();
    }

    print!("{output}");
}

fn insert(input: i32, key_table: &mut [Option<i32>], value_table: &mut [Option<i32>]) {
    let mut idx = input.rem_euclid(MOD as i32) as usize;

    while let Some(num) = key_table[idx] {
        if num == input {
            break;
        }

        idx = (idx + 1) % MOD;
    }

    key_table[idx] = Some(input);
    let value = value_table[idx].get_or_insert(0);

    *value += 1;
}

fn get(input: i32, key_table: &[Option<i32>], value_table: &[Option<i32>]) -> Option<i32> {
    let mut idx = input.rem_euclid(MOD as i32) as usize;

    while let Some(num) = key_table[idx] {
        if num == input {
            break;
        }

        idx = (idx + 1) % MOD;
    }

    value_table[idx]
}
