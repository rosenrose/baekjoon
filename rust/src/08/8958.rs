use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for input in buf.lines().skip(1) {
        let score: usize = input
            .split('X')
            .flat_map(|s| s.char_indices().map(|(i, _)| i + 1))
            .sum();

        println!("{score}");
    }
}

/*
fn group_by_o(input: &str) -> Vec<Vec<char>> {
    let mut group_of_o = Vec::new();
    let mut arr = Vec::new();

    for ch in input.chars() {
        match ch {
            'O' => arr.push(ch),
            'X' => {
                if !arr.is_empty() {
                    group_of_o.push(arr.clone());
                    arr.clear();
                }
            },
            _ => (),
        }
    }

    if !arr.is_empty() {
        group_of_o.push(arr);
    }

    group_of_o
}
*/
