use std::fmt::Write;
use std::io;
use std::io::{self, BufRead, Write};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::BufWriter::new(io::stdout().lock());

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    stdin.read_line(&mut buf).unwrap();
    writeln!(stdout, "").unwrap();

    let buf = io::read_to_string(io::stdin()).unwrap();
}

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     writeln!(stdout, "{}", a + b)?;
//     Ok(())
// }

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}

use std::string::ToString;

fn vec_join<T>(vec: &[T], seperator: &str) -> String
where
    T: ToString,
{
    vec.iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join(seperator)
}

use std::fmt::Debug;
use std::str::FromStr;

fn parse_num<T>(buf: &String) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    buf.trim().parse().unwrap()
}

fn parse_numbers<T>(buf: &String) -> impl Iterator<Item = T> + '_
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    buf.split_whitespace().map(|s| s.parse::<T>().unwrap())
}

fn from_range<R>(range: R) -> i32
where
    R: RangeBounds<i32> + std::iter::Iterator,
    <R as Iterator>::Item: DivAssign<i32>,
    <R as Iterator>::Item: Rem<i32>,
    <<R as Iterator>::Item as Rem<i32>>::Output: PartialEq<i32>,
    <R as Iterator>::Item: Copy,
{
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn print_size_of<T>(_: &T) {
    println!("{}", std::mem::size_of::<T>());
}
