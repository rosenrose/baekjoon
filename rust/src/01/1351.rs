use std::collections::HashMap;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n, p, q] = parse_int_vec(&buf)[..] else { return };
    let mut memo = HashMap::from([(0, 1)]);

    println!("{}", infinite_array(n, p, q, &mut memo));
}

fn infinite_array(n: i64, p: i64, q: i64, memo: &mut HashMap<i64, i64>) -> i64 {
    if n == 0 {
        return 1;
    }

    let mut get_or_insert = |num: i64| {
        if let Some(&ret) = memo.get(&num) {
            ret
        } else {
            let ret = infinite_array(num, p, q, memo);
            memo.insert(num, ret);

            ret
        }
    };

    get_or_insert(n / p) + get_or_insert(n / q)
}

fn parse_int_vec(buf: &String) -> Vec<i64> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
