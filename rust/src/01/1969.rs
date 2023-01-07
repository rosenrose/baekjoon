use std::collections::HashMap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    input.next();

    let m: usize = input.next().unwrap().parse().unwrap();
    let dna_strings: Vec<_> = input.map(|s| s.as_bytes()).collect();

    let mut dist_sum = 0;

    let closest_dna: String = (0..m)
        .map(|i| {
            let mut max_count = 1;
            let atgc_count = dna_strings.iter().fold(HashMap::new(), |mut acc, dna| {
                acc.entry(dna[i])
                    .and_modify(|c: &mut i32| {
                        *c += 1;
                        max_count = max_count.max(*c);
                    })
                    .or_insert(1);

                acc
            });

            let (&most_char, _) = atgc_count
                .iter()
                .filter(|&(_, v)| *v == max_count)
                .min_by_key(|&(k, _)| k)
                .unwrap();

            dist_sum += dna_strings.iter().filter(|dna| dna[i] != most_char).count();

            most_char as char
        })
        .collect();

    println!("{closest_dna}\n{dist_sum}");
}
