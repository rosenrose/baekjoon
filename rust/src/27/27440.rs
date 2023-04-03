use std::collections::HashMap;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i64 = buf.trim().parse().unwrap();
    let mut memo = HashMap::from([(1, 0), (2, 1), (3, 1)]);

    println!("{}", make_1_count(n, &mut memo));
}

fn make_1_count(n: i64, memo: &mut HashMap<i64, i32>) -> i32 {
    if let Some(count) = memo.get(&n) {
        return *count;
    }

    let mut list = Vec::new();

    if n % 3 == 0 {
        list.push(make_1_count(n / 3, memo) + 1);
    }
    if n % 2 == 0 {
        list.push(make_1_count(n / 2, memo) + 1);
    }
    if (n - 1) % 3 == 0 || (n - 1) % 2 == 0 {
        list.push(make_1_count(n - 1, memo) + 1);
    }
    if (n - 2) % 3 == 0 {
        list.push(make_1_count(n - 2, memo) + 2);
    }

    let count = *list.iter().min().unwrap();
    memo.insert(n, count);

    count
}
