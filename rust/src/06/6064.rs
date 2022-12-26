use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    for _ in 0..input() {
        let (m, n, x, y) = (input(), input(), input(), input());
        let (gcd, mut a, mut b) = get_ex_gcd(m, n);
        // println!("{a} {b} {gcd}");
        if (y - x) % gcd != 0 {
            writeln!(output, "-1").unwrap();
            continue;
        }

        let multiple = (y - x) / gcd;
        let (b_step, a_step) = (m / gcd, n / gcd);

        a *= multiple;
        b *= -multiple;

        while a > 0 || b > 0 {
            a -= a_step;
            b -= b_step;
        }
        while a < 0 || b < 0 {
            a += a_step;
            b += b_step;
        }
        // println!("{a} {b}");
        // assert_eq!(m*a+x, n*b+y);
        // println!("{} {}", m * a + x, n * b + y);

        writeln!(output, "{}", m * a + x).unwrap();
    }

    print!("{output}");
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

// use std::fmt::Write;
// use std::io::{stdin, Read};

// fn main() {
//     let mut buf = String::new();
//     stdin().read_to_string(&mut buf).unwrap();

//     let mut input = buf
//         .split_ascii_whitespace()
//         .map(|s| s.parse::<usize>().unwrap());
//     let mut input = || input.next().unwrap();
//     let mut output = String::new();

//     'outer: for _ in 0..input() {
//         let (m, n, x, y) = (input(), input(), input(), input());
//         let total_years = get_lcm(m, n);

//         let (start_year, compare_year) = if m > n {
//             (x, if y == n { 0 } else { y })
//         } else {
//             (y, if x == m { 0 } else { x })
//         };

//         let (mod1, mod2) = (m.max(n), m.min(n));

//         for year in (start_year..=total_years).step_by(mod1) {
//             if year % mod2 == compare_year {
//                 writeln!(output, "{year}").unwrap();
//                 continue 'outer;
//             }
//         }

//         writeln!(output, "-1").unwrap();
//     }

//     print!("{output}");
// }

// fn get_lcm(a: usize, b: usize) -> usize {
//     a / get_gcd(a, b) * b
// }

// fn get_gcd(mut a: usize, mut b: usize) -> usize {
//     loop {
//         if b == 0 {
//             return a;
//         }

//         (a, b) = (b, a % b);
//     }
// }
