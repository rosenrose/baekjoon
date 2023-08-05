use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let [_, m] = [(); 2].map(|_| input.next().unwrap().parse::<usize>().unwrap());
    let dna_strings: Vec<_> = input.map(str::as_bytes).collect();

    let mut dist_sum = 0;

    let closest_dna: String = (0..m)
        .map(|i| {
            let mut max_count = 1;
            let mut acgt_count = [0; 4];

            for dna in &dna_strings {
                let idx = match dna[i] as char {
                    'A' => 0,
                    'C' => 1,
                    'G' => 2,
                    'T' => 3,
                    _ => unreachable!(),
                };

                acgt_count[idx] += 1;
                max_count = acgt_count[idx].max(max_count);
            }

            let most_char_idx = acgt_count
                .iter()
                .enumerate()
                .filter_map(|(i, &count)| (count == max_count).then_some(i))
                .min()
                .unwrap();

            let most_char = match most_char_idx {
                0 => 'A',
                1 => 'C',
                2 => 'G',
                3 => 'T',
                _ => unreachable!(),
            };

            dist_sum += dna_strings
                .iter()
                .filter(|dna| dna[i] != most_char as u8)
                .count();

            most_char
        })
        .collect();

    println!("{closest_dna}\n{dist_sum}");
}
