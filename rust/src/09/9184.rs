use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let mut memo = [[[0; 21]; 21]; 21];

    while let [Some(a), Some(b), Some(c)] = [(); 3].map(|_| input.next()) {
        if [a, b, c] == [-1; 3] {
            break;
        }

        writeln!(output, "w({a}, {b}, {c}) = {}", w(a, b, c, &mut memo)).unwrap();
    }

    print!("{output}");
}

fn w(a: i32, b: i32, c: i32, memo: &mut [[[i32; 21]; 21]]) -> i32 {
    if a <= 0 || b <= 0 || c <= 0 {
        return 1;
    }

    if a > 20 || b > 20 || c > 20 {
        return w(20, 20, 20, memo);
    }

    let (a_idx, b_idx, c_idx) = (a as usize, b as usize, c as usize);

    if memo[a_idx][b_idx][c_idx] != 0 {
        return memo[a_idx][b_idx][c_idx];
    }

    memo[a_idx][b_idx][c_idx] = if a < b && b < c {
        w(a, b, c - 1, memo) + w(a, b - 1, c - 1, memo) - w(a, b - 1, c, memo)
    } else {
        w(a - 1, b, c, memo) + w(a - 1, b - 1, c, memo) + w(a - 1, b, c - 1, memo)
            - w(a - 1, b - 1, c - 1, memo)
    };

    memo[a_idx][b_idx][c_idx]
}
