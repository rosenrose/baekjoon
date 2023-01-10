use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().next_back().unwrap();

    let id_scores: Vec<_> = ["Adrian", "Bruno", "Goran"]
        .iter()
        .map(|&id| {
            let score: i32 = input
                .char_indices()
                .map(|(i, ch)| match id {
                    "Adrian" => match (i % 3, ch) {
                        (0, 'A') | (1, 'B') | (2, 'C') => 1,
                        _ => 0,
                    },
                    "Bruno" => match (i % 4, ch) {
                        (0, 'B') | (1, 'A') | (2, 'B') | (3, 'C') => 1,
                        _ => 0,
                    },
                    "Goran" => match (i % 6, ch) {
                        (0 | 1, 'C') | (2 | 3, 'A') | (4 | 5, 'B') => 1,
                        _ => 0,
                    },
                    _ => 0,
                })
                .sum();

            (id, score)
        })
        .collect();

    let (_, max_score) = id_scores.iter().max_by_key(|(_, s)| s).unwrap();
    println!("{max_score}");

    for (id, _) in id_scores.iter().filter(|(_, s)| s == max_score) {
        println!("{id}");
    }
}
