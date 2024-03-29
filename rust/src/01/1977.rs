use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<i32>);

    let [m, n] = [(); 2].map(|_| input.next().unwrap());
    let square_nums: Vec<_> = (1..)
        .skip_while(|i| i * i < m)
        .map_while(|i| {
            let square = i * i;
            (square <= n).then_some(square)
        })
        .collect();

    if square_nums.is_empty() {
        println!("-1");
        return;
    }

    println!(
        "{}\n{}",
        square_nums.iter().sum::<i32>(),
        square_nums.iter().min().unwrap()
    );
}
