use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<usize>);

    const MAX: i32 = 21;
    let mut cache = [[-1; 5]; MAX as usize + 1];

    get_count(MAX, 0, &mut cache);
    // println!("{cache:?}");
    for n in input.skip(1) {
        println!("{}", cache[n][0]);
    }
}

fn get_count(n: i32, state: usize, cache: &mut [[i32; 5]; 22]) -> i32 {
    if n < 0 {
        return 0;
    }
    if n == 0 {
        return if state == 0 { 1 } else { 0 };
    }

    let mut count = cache[n as usize][state];

    if count != -1 {
        return count;
    }

    count = match state {
        0 => {
            get_count(n - 1, 0, cache)
                + get_count(n - 1, 1, cache)
                + get_count(n - 1, 3, cache)
                + get_count(n - 1, 4, cache)
                + get_count(n - 2, 0, cache)
        }
        1 => get_count(n - 1, 0, cache) + get_count(n - 1, 4, cache),
        2 => get_count(n - 1, 3, cache),
        3 => get_count(n - 1, 0, cache) + get_count(n - 1, 2, cache),
        4 => get_count(n - 1, 0, cache) + get_count(n - 1, 1, cache),
        _ => Default::default(),
    };

    cache[n as usize][state] = count;
    count
}
