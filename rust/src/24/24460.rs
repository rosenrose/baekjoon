use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let chairs: Vec<Vec<_>> = (0..n).map(|_| input.by_ref().take(n).collect()).collect();

    println!("{}", get_2nd_small(&chairs, n, 0, 0));
}

fn get_2nd_small(chairs: &[Vec<i32>], n: usize, x: usize, y: usize) -> i32 {
    if n == 1 {
        return chairs[y][x];
    }

    let half = n / 2;
    let mut nums = [
        get_2nd_small(chairs, half, x, y),
        get_2nd_small(chairs, half, x + half, y),
        get_2nd_small(chairs, half, x, y + half),
        get_2nd_small(chairs, half, x + half, y + half),
    ];

    *nums.select_nth_unstable(1).1
}
