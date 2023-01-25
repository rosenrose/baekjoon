use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let (n, m) = (input(), input());
    let a: Vec<Vec<_>> = (0..n).map(|_| (0..m).map(|_| input()).collect()).collect();
    let (_, k) = (input(), input());
    let b: Vec<Vec<_>> = (0..m).map(|_| (0..k).map(|_| input()).collect()).collect();

    for a_row in a {
        for i in 0..k as usize {
            let sum: i32 = a_row
                .iter()
                .enumerate()
                .map(|(j, a_num)| a_num * b[j][i])
                .sum();

            print!("{sum} ");
        }

        println!("");
    }
}
