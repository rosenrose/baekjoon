const M: i32 = 10007;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [n, k] = parse_int_vec(&buf)[..] {
        println!("{}", combination_num_rem(n, k));
    }
}

fn combination_num_rem(n: i32, r: i32) -> i32 {
    if n == r || r == 0 {
        return 1;
    }

    let mul_rem = |acc, i| (acc % M * i % M) % M;

    let a = (n - r + 1..=n).fold(1, mul_rem);
    let b = mod_inverse((1..=r).fold(1, mul_rem), M).unwrap();

    (a % M * b % M) % M
}

fn mod_inverse(n: i32, modular: i32) -> Option<i32> {
    let (gcd, inverse, _) = get_ex_gcd(n, modular);

    if gcd != 1 {
        return None;
    }

    Some(inverse)
}

fn get_ex_gcd(a: i32, b: i32) -> (i32, i32, i32) {
    let (mut r1, mut r2) = (a, b);
    let (mut s1, mut s2) = (1, 0);
    let (mut t1, mut t2) = (0, 1);
    let mut q;

    loop {
        q = r1 / r2;
        (r1, r2) = (r2, r1 % r2);

        (s1, s2) = (s2, s1 - s2 * q);
        (t1, t2) = (t2, t1 - t2 * q);

        if r2 == 0 {
            if s1 < 0 {
                s1 += b;
                t1 -= a;
            }

            return (r1, s1, t1);
        }
    }
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

// fn combination_num_rem(n: i32, r: i32, cache: &mut HashMap<(i32, i32), i32>) -> i32 {
//     if n == r || r == 0 {
//         return 1;
//     }

//     let mut get_or_insert = |n: i32, r: i32| match cache.get(&(n, r)) {
//         Some(i) => *i,
//         None => {
//             let ret = combination_num_rem(n, r, cache);
//             cache.insert((n, r), ret % M);

//             ret
//         }
//     };

//     (get_or_insert(n - 1, r - 1) + get_or_insert(n - 1, r)) % M
// }
