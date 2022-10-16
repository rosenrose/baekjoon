use std::collections::{BTreeSet, HashMap};
use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();
    buf.clear();
    stdin.read_line(&mut buf).unwrap();

    let mut nums = Vec::new();
    let mut nums_sorted_set = BTreeSet::new();

    for token in buf.split_whitespace() {
        let n: i32 = token.parse().unwrap();

        nums.push(n);
        nums_sorted_set.insert(n);
    }

    let compressed: HashMap<i32, usize> = nums_sorted_set
        .iter()
        .enumerate()
        .map(|(i, &num)| (num, i))
        .collect();

    for num in nums {
        write!(stdout, "{} ", compressed.get(&num).unwrap()).unwrap();
    }
}
