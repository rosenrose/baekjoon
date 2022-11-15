use std::collections::HashMap;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();
    let mut table = HashMap::from([(0, 0), (1, 0), (2, 1), (3, 1)]);

    println!("{}", make_1_count(n, &mut table));
}

fn make_1_count(n: i32, table: &mut HashMap<i32, i32>) -> i32 {
    match table.get(&n) {
        Some(count) => *count,
        _ => {
            let mut list = Vec::new();

            if n % 3 == 0 {
                list.push(make_1_count(n / 3, table));
            }
            if n % 2 == 0 {
                list.push(make_1_count(n / 2, table));
            }
            if (n - 1) % 3 == 0 || (n - 1) % 2 == 0 {
                list.push(make_1_count(n - 1, table));
            }
            if (n - 2) % 3 == 0 {
                list.push(make_1_count(n - 2, table) + 1);
            }
            // println!("{n} {list:?}");
            let count = *list.iter().min().unwrap() + 1;
            table.insert(n, count);

            count
        }
    }
}