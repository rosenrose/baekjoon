use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let file_names: Vec<_> = buf.lines().skip(1).collect();

    if file_names.len() == 1 {
        println!("{}", file_names[0]);
        return;
    }

    let pattern: String = (0..file_names[0].len())
        .map(|i| {
            let letter = file_names[0].chars().nth(i);

            let is_same = file_names[1..]
                .iter()
                .map(|file_name| file_name.chars().nth(i))
                .all(|c| c == letter);

            if is_same {
                letter.unwrap()
            } else {
                '?'
            }
        })
        .collect();

    println!("{pattern}");
}
