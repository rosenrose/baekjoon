use std::collections::HashMap;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut output = String::new();

    let n: i32 = input.next().unwrap().parse().unwrap();
    input.next();

    let mut pokemon_name = HashMap::new();
    let mut pokemon_index = Vec::new();

    for i in 1..=n {
        let name = input.next().unwrap();

        pokemon_name.insert(name, i);
        pokemon_index.push(name);
    }

    for query in input {
        (match query.parse::<usize>() {
            Ok(i) => writeln!(output, "{}", pokemon_index[i - 1]),
            _ => writeln!(output, "{}", pokemon_name.get(query).unwrap()),
        })
        .unwrap();
    }

    print!("{output}");
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
//     Err(_) => Answer::Index(*pokemons_map.get(input).unwrap()),
// };
