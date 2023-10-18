use std::collections::HashMap;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut output = String::new();

    let [n, _] = [(); 2].map(|_| input.next().unwrap().parse::<usize>().unwrap());
    let mut pokemon_index = Vec::with_capacity(n);
    let mut pokemon_name = HashMap::with_capacity(n);

    for (i, name) in input.by_ref().take(n).enumerate() {
        pokemon_index.push(name);
        pokemon_name.insert(name, i as i32 + 1);
    }

    for query in input {
        if let Ok(i) = query.parse::<usize>() {
            writeln!(output, "{}", pokemon_index[i - 1])
        } else {
            writeln!(output, "{}", pokemon_name[query])
        }
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
//     Err(_) => Answer::Index(*pokemons_map[input]),
// };
