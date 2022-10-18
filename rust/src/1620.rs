use std::collections::HashMap;
use std::fmt;
use std::io::{stdin, stdout, BufRead, BufWriter, Write};

enum Answer<'a> {
    Name(&'a String),
    Index(usize),
}

impl fmt::Display for Answer<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Answer::Name(name) => write!(f, "{name}"),
            Answer::Index(index) => write!(f, "{index}"),
        }
    }
}

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    if let [n, m] = parse_int_vec(&buf)[..] {
        let mut pokemons_map = HashMap::new();
        let mut pokemons_arr = Vec::new();

        for i in 1..=n {
            buf.clear();
            stdin.read_line(&mut buf).unwrap();
            let name = buf.trim();

            pokemons_map.insert(name.to_string(), i as usize);
            pokemons_arr.push(name.to_string());
        }

        for _ in 0..m {
            buf.clear();
            stdin.read_line(&mut buf).unwrap();
            let input = buf.trim();

            let answer = match input.parse::<usize>() {
                Ok(index) => Answer::Name(&pokemons_arr[index - 1]),
                Err(_) => Answer::Index(*pokemons_map.get(input).unwrap()),
            };

            writeln!(stdout, "{answer}").unwrap();
        }
    }
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
