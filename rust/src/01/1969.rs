use std::collections::HashMap;

fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n = buf
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .next()
        .unwrap();
    let dna_strings = parse_str_vec_lines(&mut buf, n);

    let mut dist_sum = 0;
    let mut atgc_count = HashMap::new();

    let closest_dna: String = (0..dna_strings[0].len())
        .map(|i| {
            atgc_count.clear();
            let mut max_count = 1;

            dna_strings
                .iter()
                .map(|s| s.chars().nth(i).unwrap())
                .for_each(|c| {
                    atgc_count
                        .entry(c)
                        .and_modify(|count: &mut i32| {
                            *count += 1;
                            max_count = (*count).max(max_count);
                        })
                        .or_insert(1);
                });

            let (most_char, _) = atgc_count
                .iter()
                .filter(|&(_, v)| *v == max_count)
                .min_by_key(|&(k, _)| k)
                .unwrap();

            dist_sum += dna_strings
                .iter()
                .filter(|s| s.chars().nth(i).unwrap() != *most_char)
                .count();

            *most_char
        })
        .collect();

    println!("{closest_dna}\n{dist_sum}");
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_str_vec_lines(buf: &mut String, n: usize) -> Vec<String> {
    (0..n)
        .map(|_| {
            read_line(buf);
            buf.trim().to_string()
        })
        .collect()
}
