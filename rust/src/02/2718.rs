use std::io;

const MAX: usize = 21;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<usize>);

    let mut memo = [[-1; 5]; MAX + 1];

    get_count(MAX as i32, 0, &mut memo);
    // println!("{memo:?}");
    for n in input.skip(1) {
        println!("{}", memo[n][0]);
    }
}

fn get_count(n: i32, state: usize, memo: &mut [[i32; 5]]) -> i32 {
    if n < 0 {
        return 0;
    }
    if n == 0 {
        return (state == 0) as i32;
    }

    let mut count = memo[n as usize][state];

    if count != -1 {
        return count;
    }

    count = match state {
        0 => {
            get_count(n - 1, 0, memo)
                + get_count(n - 1, 1, memo)
                + get_count(n - 1, 3, memo)
                + get_count(n - 1, 4, memo)
                + get_count(n - 2, 0, memo)
        }
        1 => get_count(n - 1, 0, memo) + get_count(n - 1, 4, memo),
        2 => get_count(n - 1, 3, memo),
        3 => get_count(n - 1, 0, memo) + get_count(n - 1, 2, memo),
        4 => get_count(n - 1, 0, memo) + get_count(n - 1, 1, memo),
        _ => unreachable!(),
    };

    memo[n as usize][state] = count;
    count
}

// let mut memo = [0; MAX + 1];
// memo[0..3].copy_from_slice(&[1, 1, 5]);

// for i in 3..=21 {
//     memo[i] = memo[i - 1]
//         + memo[i - 2] * 4
//         + (0..=i - 3)
//             .rev()
//             .map(|j| {
//                 memo[j]
//                     * if i & 1 == 0 {
//                         if j & 1 == 0 {
//                             3
//                         } else {
//                             2
//                         }
//                     } else {
//                         if j & 1 == 0 {
//                             2
//                         } else {
//                             3
//                         }
//                     }
//             })
//             .sum::<i32>();
