use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    if let [birth_year, birth_month, birth_date, cmp_year, cmp_month, cmp_date] =
        input.collect::<Vec<_>>()[..]
    {
        let age = cmp_year - birth_year;
        let is_early = (cmp_month, cmp_date) < (birth_month, birth_date);

        println!("{}", age - if is_early { 1 } else { 0 });
        println!("{}", age + 1);
        println!("{}", age);
    }
}
