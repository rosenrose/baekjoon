use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [n, k] = [(); 2].map(|_| input.next().unwrap());
    let mut students = [[0_usize, 0]; 7];

    for [gender, grade] in (0..n).map(|_| [(); 2].map(|_| input.next().unwrap())) {
        students[grade][gender] += 1;
    }

    let count: usize = students[1..]
        .iter()
        .map(|&[f, m]| f.div_ceil(k) + m.div_ceil(k))
        .sum();

    println!("{count}");
}
