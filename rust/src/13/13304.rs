use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [n, k] = [(); 2].map(|_| input.next().unwrap());
    let mut students = [[0_usize, 0]; 7];

    for [gender, grade] in (0..n).map(|_| [(); 2].map(|_| input.next().unwrap())) {
        students[grade][gender] += 1;
    }

    let mut sum: usize = students[1..=2]
        .iter()
        .map(|row| row.iter().sum::<usize>())
        .sum();
    let mut count = sum.div_ceil(k);

    for gender in 0..=1 {
        sum = (3..=4).map(|grade| students[grade][gender]).sum();
        count += sum.div_ceil(k);

        sum = (5..=6).map(|grade| students[grade][gender]).sum();
        count += sum.div_ceil(k);
    }

    println!("{count}");
}
