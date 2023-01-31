use std::collections::HashMap;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();
    let mut cache = HashMap::from([(1, 0), (2, 1), (3, 1)]);

    println!("{}", make_1_count(n, &mut cache));
}

fn make_1_count(n: i32, cache: &mut HashMap<i32, i32>) -> i32 {
    if let Some(count) = cache.get(&n) {
        return *count;
    }

    let mut list = Vec::new();

    if n % 3 == 0 {
        list.push(make_1_count(n / 3, cache) + 1);
    }
    if n % 2 == 0 {
        list.push(make_1_count(n / 2, cache) + 1);
    }
    if (n - 1) % 3 == 0 || (n - 1) % 2 == 0 {
        list.push(make_1_count(n - 1, cache) + 1);
    }
    if (n - 2) % 3 == 0 {
        list.push(make_1_count(n - 2, cache) + 2);
    }
    // println!("{n} {list:?}");
    let count = *list.iter().min().unwrap();
    cache.insert(n, count);

    count
}
