fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let birth = parse_int_vec(&buf);
    read_line(&mut buf);

    let compare = parse_int_vec(&buf);

    if let [birth_year, birth_month, birth_date, compare_year, compare_month, compare_date] =
        [birth, compare].concat()[..]
    {
        let age = compare_year - birth_year;
        let is_early = (compare_month, compare_date) < (birth_month, birth_date);

        println!("{}", age - if is_early { 1 } else { 0 });
        println!("{}", age + 1);
        println!("{}", age);
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
