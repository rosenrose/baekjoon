use std::io;

const MAX: usize = 50;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let n: usize = input.next().unwrap().parse().unwrap();
    let mut file_names = [""; MAX];

    for (i, name) in input.enumerate() {
        file_names[i] = name;
    }

    if n == 1 {
        println!("{}", file_names[0]);
        return;
    }

    let pattern: String = (0..file_names[0].len())
        .map(|i| {
            let letter = file_names[0].chars().nth(i).unwrap();
            let is_same = file_names[1..n]
                .iter()
                .flat_map(|file_name| file_name.chars().nth(i))
                .all(|ch| ch == letter);

            if is_same {
                letter
            } else {
                '?'
            }
        })
        .collect();

    println!("{pattern}");
}
