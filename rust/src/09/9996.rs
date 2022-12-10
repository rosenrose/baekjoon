use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.lines();
    input.next();

    let mut pattern = input.next().unwrap().split('*');
    let (prefix, suffix) = (pattern.next().unwrap(), pattern.next().unwrap());

    for file_name in input {
        if file_name.len() < prefix.len() + suffix.len() {
            println!("NE");
            continue;
        }

        if file_name.starts_with(prefix) && file_name.ends_with(suffix) {
            println!("DA");
        } else {
            println!("NE");
        }
    }
}
