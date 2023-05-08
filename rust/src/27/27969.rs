fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    const DEFAULT: usize = 8;
    let mut sizes = [0; 7501];
    let (mut depth, mut max_depth) = (0, 0);

    for token in buf.trim().split(' ') {
        match token {
            "[" => {
                depth += 1;
                max_depth = depth.max(max_depth)
            }
            "]" => {
                depth -= 1;
                sizes[depth] += DEFAULT;
            }
            s if s.starts_with(|ch: char| ch.is_ascii_digit()) => sizes[depth] += 8,
            s if s.starts_with(|ch: char| ch.is_ascii_alphabetic()) => {
                sizes[depth] += token.len() + 12
            }
            _ => unreachable!(),
        }
    }
    // println!("{:?}", &sizes[1..=max_depth]);
    println!("{}", sizes[1..=max_depth].iter().sum::<usize>() + DEFAULT);
}
