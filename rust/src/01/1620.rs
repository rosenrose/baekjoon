use std::collections::HashMap;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (n, m) = (parse_int(input()), parse_int(input()));
    let mut pokemon_name = HashMap::new();
    let mut pokemon_index = Vec::new();

    for (i, name) in (1..=n).map(|i| (i, input())) {
        pokemon_name.insert(name, i);
        pokemon_index.push(name);
    }

    for query in (0..m).map(|_| input()) {
        (if let Ok(i) = query.parse::<usize>() {
            writeln!(output, "{}", pokemon_index[i - 1])
        } else {
            writeln!(output, "{}", pokemon_name[query])
        })
        .unwrap();
    }

    print!("{output}");
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}

// enum Answer<'a> {
//     Name(&'a String),
//     Index(usize),
// }

// impl fmt::Display for Answer<'_> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Answer::Name(name) => write!(f, "{name}"),
//             Answer::Index(index) => write!(f, "{index}"),
//         }
//     }
// }

// let answer = match input.parse::<usize>() {
//     Ok(index) => Answer::Name(&pokemons_arr[index - 1]),
//     Err(_) => Answer::Index(*pokemons_map[input]),
// };
