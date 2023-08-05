use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut output = String::new();

    while let [Some(n), Some(m)] = [(); 2].map(|_| input.next()) {
        let count = (n..=m)
            .filter(|num| {
                let mut num = *num;
                let mut visited = [false; 10];

                while num > 0 {
                    if visited[num % 10] {
                        return false;
                    }

                    visited[num % 10] = true;
                    num /= 10;
                }

                true
            })
            .count();

        writeln!(output, "{count}").unwrap();
    }

    print!("{output}");
}
