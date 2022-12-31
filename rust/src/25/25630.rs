use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf.lines().next_back().unwrap().as_bytes();

    let (mut i, mut j) = (0, input.len() - 1);
    let mut count = 0;

    loop {
        if i >= j {
            break;
        }

        if input[i] != input[j] {
            count += 1;
        }

        i += 1;
        j -= 1;
    }

    println!("{count}");
}
