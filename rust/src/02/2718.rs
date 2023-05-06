use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<usize>);

    const MAX: i32 = 21;
    let mut memo = [[-1; 5]; MAX as usize + 1];

    get_count(MAX, 0, &mut memo);
    // println!("{memo:?}");
    for n in input.skip(1) {
        println!("{}", memo[n][0]);
    }
}

fn get_count(n: i32, state: usize, memo: &mut [[i32; 5]; 22]) -> i32 {
    if n < 0 {
        return 0;
    }
    if n == 0 {
        return i32::from(state == 0);
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
