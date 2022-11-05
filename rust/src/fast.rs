#![allow(
    unused_macros,
    clippy::needless_collect,
    clippy::unnecessary_lazy_evaluations
)]
use std::io::{self, *};
use std::str::FromStr;

struct Reader {
    stdin: Stdin,
}

#[allow(dead_code)]
impl Reader {
    fn new() -> Self {
        Self { stdin: io::stdin() }
    }

    fn lines(&self) -> impl Iterator<Item = String> + '_ {
        BufReader::new(self.stdin.lock())
            .lines()
            .map(Result::unwrap)
    }

    fn string(self) -> String {
        let mut s = String::new();
        self.stdin.lock().read_to_string(&mut s).unwrap();
        s
    }

    fn words(self) -> impl Iterator<Item = &'static str> {
        let s = Box::leak(self.string().into_boxed_str());
        s.split_ascii_whitespace()
    }

    fn numbers<T>(self) -> impl Iterator<Item = T>
    where
        T: FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.words().map(|x| x.parse::<T>().unwrap())
    }
}

fn main() {
    let mut writer = BufWriter::new(stdout());
    macro_rules! p { ($($e:expr),*  $(,)? ) => { write!(writer, $($e),* ).unwrap() } }
    macro_rules! puts { ( $($e:expr),* $(,)? ) => { writeln!(writer, $($e),* ).unwrap() } }
    let reader = Reader::new();

    let mut it = reader.numbers::<isize>();
    let n = it.next().unwrap() as usize;
    let m = it.next().unwrap() as usize;

    let array: Vec<Vec<_>> = (0..n)
        .map(|_| {
            let mut row = vec![0];
            for x in it.by_ref().take(m) {
                row.push(row[row.len() - 1] + x);
            }
            row
        })
        .collect();
    let k = it.next().unwrap() as usize;
    for _ in 0..k {
        let i = it.next().unwrap() as usize - 1;
        let j = it.next().unwrap() as usize - 1;
        let x = it.next().unwrap() as usize;
        let y = it.next().unwrap() as usize;

        let ans = (i..x).map(|r| array[r][y] - array[r][j]).sum::<isize>();
        puts!("{ans}");
    }
}
