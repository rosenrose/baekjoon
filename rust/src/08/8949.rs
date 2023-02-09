fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let parse_num = |s: &str| -> Vec<u8> { s.chars().rev().map(|c| c as u8 - '0' as u8).collect() };
    let mut input = buf.split_whitespace().map(parse_num);

    let (a, b) = (input.next().unwrap(), input.next().unwrap());
    let sum = (0..a.len().max(b.len())).map(|i| a.get(i).unwrap_or(&0) + b.get(i).unwrap_or(&0));

    for i in sum.rev() {
        print!("{i}");
    }
}
