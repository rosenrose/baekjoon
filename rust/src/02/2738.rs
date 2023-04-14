use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let (n, m) = (input.next().unwrap(), input.next().unwrap() as usize);
    let mut parse_matrix = |n: i32, m: usize| -> Vec<Vec<_>> {
        (0..n).map(|_| input.by_ref().take(m).collect()).collect()
    };

    let (a, b) = (parse_matrix(n, m), parse_matrix(n, m));
    let sum = a
        .iter()
        .zip(b)
        .map(|(row1, row2)| row1.iter().zip(row2).map(|(col1, col2)| col1 + col2));

    for row in sum {
        for col in row {
            print!("{col} ");
        }
        println!("");
    }
}
