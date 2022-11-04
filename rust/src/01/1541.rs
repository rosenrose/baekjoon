fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let formula_sum = |formula: &str| formula.split('+').map(|s| s.parse::<i32>().unwrap()).sum();

    let mut splitted = buf.trim().split('-');
    let first: i32 = formula_sum(splitted.next().unwrap());

    let result = splitted.fold(first, |a, b| a - formula_sum(b));

    println!("{result}");
}
