use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
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
