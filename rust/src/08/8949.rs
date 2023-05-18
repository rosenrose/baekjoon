fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let parse_num =
        |s: &str| -> Vec<u8> { s.as_bytes().iter().rev().map(|ch| ch - b'0').collect() };
    let mut input = buf.split_whitespace().map(parse_num);

    let (a, b) = (input.next().unwrap(), input.next().unwrap());
    let sum = (0..a.len().max(b.len())).map(|i| a.get(i).unwrap_or(&0) + b.get(i).unwrap_or(&0));

    for num in sum.rev() {
        print!("{num}");
    }
}
