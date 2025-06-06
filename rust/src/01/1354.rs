use std::collections::HashMap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let [n, p, q, x, y] = [(); 5].map(|_| input.next().unwrap());
    let mut memo = HashMap::from([(0, 1)]);

    println!("{}", infinite_array(n, p, q, x, y, &mut memo));
}

fn infinite_array(n: i64, p: i64, q: i64, x: i64, y: i64, memo: &mut HashMap<i64, i64>) -> i64 {
    if n <= 0 {
        return 1;
    }

    let mut get_or_insert = |num: i64| {
        if let Some(&ret) = memo.get(&num) {
            ret
        } else {
            let ret = infinite_array(num, p, q, x, y, memo);
            memo.insert(num, ret);

            ret
        }
    };

    let (a, b) = ((n / p) - x, (n / q) - y);

    return if a > 0 { get_or_insert(a) } else { 1 } + if b > 0 { get_or_insert(b) } else { 1 };
}
