use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [birth_year, birth_month, birth_date, cmp_year, cmp_month, cmp_date] =
        [(); 6].map(|_| input.next().unwrap());

    let age = cmp_year - birth_year;
    let is_early = (cmp_month, cmp_date) < (birth_month, birth_date);

    println!("{}\n{}\n{age}", age - i32::from(is_early), age + 1);
}
