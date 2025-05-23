use std::collections::HashMap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let n: i32 = buf.trim().parse().unwrap();

    let mut memo = HashMap::from([
        (1, (0, vec![1])),
        (2, (1, vec![1, 2])),
        (3, (1, vec![1, 3])),
    ]);

    let (count, path) = make_1_count(n, &mut memo);

    println!("{count}");

    for p in path.iter().rev() {
        print!("{p} ");
    }
}

fn make_1_count(n: i32, memo: &mut HashMap<i32, (i32, Vec<i32>)>) -> (i32, Vec<i32>) {
    if let Some(count_path) = memo.get(&n) {
        return (*count_path).clone();
    }

    let mut list = Vec::new();
    let (mut count, mut path);

    if n % 3 == 0 {
        (count, path) = make_1_count(n / 3, memo);
        list.push((count + 1, path));
    }
    if n % 2 == 0 {
        (count, path) = make_1_count(n / 2, memo);
        list.push((count + 1, path));
    }
    if (n - 1) % 3 == 0 || (n - 1) % 2 == 0 {
        (count, path) = make_1_count(n - 1, memo);
        list.push((count + 1, path));
    }
    if (n - 2) % 3 == 0 {
        (count, path) = make_1_count(n - 2, memo);
        path.push(n - 1);
        list.push((count + 2, path));
    }
    // println!("{n} {list:?}");
    let (count, mut path) = list.iter().min().unwrap().clone();

    path.push(n);
    memo.insert(n, (count, path.clone()));

    (count, path)
}
