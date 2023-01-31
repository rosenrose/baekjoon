use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for input in buf.lines().skip(1) {
        let score: usize = input
            .split(|c| c == 'X')
            .flat_map(|s| s.char_indices().map(|(i, _)| i + 1))
            .sum();

        println!("{score}");
    }
}

/*
fn group_by_o(str: &String) -> Vec<Vec<char>> {
    let mut group_of_o = Vec::new();
    let mut arr = Vec::new();

    for char in str.chars() {
        match char {
            'O' => arr.push(char),
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
