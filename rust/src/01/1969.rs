use std::collections::HashMap;

fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    if let [n, m] = parse_int_vec(&buf)[..] {
        let dna_strings = parse_str_vec_lines(&mut buf, n);
        let mut dist_sum = 0;
        let mut atgc_count = HashMap::new();

        let closest_dna: String = (0..m)
            .map(|i| {
                atgc_count.clear();
                let (mut max_count, mut most_char) = (1, 'T');

                dna_strings
                    .iter()
                    .map(|s| s.chars().nth(i).unwrap())
                    .for_each(|c| {
                        atgc_count
                            .entry(c)
                            .and_modify(|count| {
                                *count += 1;

                                if *count == max_count {
                                    most_char = c.min(most_char);
                                }

                                if *count > max_count {
                                    (max_count, most_char) = (*count, c);
                                }
                            })
                            .or_insert_with(|| {
                                most_char = c.min(most_char);

                                1
                            });
                    });

                dist_sum += dna_strings
                    .iter()
                    .filter(|s| s.chars().nth(i).unwrap() != most_char)
                    .count();

                most_char
            })
            .collect();

        println!("{closest_dna}\n{dist_sum}");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<usize> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn parse_str_vec_lines(buf: &mut String, n: usize) -> Vec<String> {
    (0..n)
        .map(|_| {
            read_line(buf);
            buf.trim().to_string()
        })
        .collect()
}
