fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();

    for _ in 0..n {
        read_line(&mut buf);

        let score: usize = buf
            .trim()
            .split(|c| c == 'X')
            .filter_map(|s| {
                if s.is_empty() {
                    None
                } else {
                    Some(s.char_indices().map(|(i, _)| i + 1))
                }
            })
            .flatten()
            .sum();

        println!("{score}");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
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
