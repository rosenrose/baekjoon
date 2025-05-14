const M: i32 = 10_007;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n, k] = parse_int_vec(&buf)[..] else {
        return;
    };

    println!("{}", combination_rem(n, k));
}

fn combination_rem(n: i32, r: i32) -> i32 {
    if n == r || r == 0 {
        return 1;
    }

    let mul_rem = |acc, i| (acc * (i % M)) % M;

    let a = (n - r + 1..=n).fold(1, mul_rem);
    let b = mod_inverse_rem((1..=r).fold(1, mul_rem), M);

    (a * b) % M
}

fn mod_inverse_rem(n: i32, modular: i32) -> i32 {
    pow_rem(n, modular - 2)
}

fn pow_rem(mut base: i32, mut exp: i32) -> i32 {
    let mut rem = 1;

    while exp > 0 {
        if exp & 1 == 1 {
            rem = (rem * base) % M;
        }

        base = (base * base) % M;
        exp >>= 1;
    }

    rem
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}

// fn combination_rem(n: i32, r: i32, memo: &mut HashMap<(i32, i32), i32>) -> i32 {
//     if n == r || r == 0 {
//         return 1;
//     }

//     let mut get_or_insert = |n: i32, r: i32| {
//         if let Some(&i) = memo.get(&(n, r)) {
//             i
//         } else {
//             let ret = combination_rem(n, r, memo);
//             memo.insert((n, r), ret);

//             ret
//         }
//     };

//     (get_or_insert(n - 1, r - 1) + get_or_insert(n - 1, r)) % M
// }
