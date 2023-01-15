fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let mut current = input.next().unwrap();

    for next in input {
        if next < current {
            println!("Bad");
            return;
        }

        current = next;
    }

    println!("Good");
}
