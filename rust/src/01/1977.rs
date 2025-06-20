use std::io;

const MAX: usize = 10000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<i32>);

    let [m, n] = [(); 2].map(|_| input.next().unwrap());
    let mut square_nums = [0; MAX];
    let mut square_nums_len = 0;

    for num in (1..).skip_while(|i| i * i < m).map_while(|i| {
        let square = i * i;
        (square <= n).then_some(square)
    }) {
        square_nums[square_nums_len] = num;
        square_nums_len += 1;
    }

    if square_nums_len == 0 {
        println!("-1");
        return;
    }

    println!(
        "{}\n{}",
        square_nums[..square_nums_len].iter().sum::<i32>(),
        square_nums[..square_nums_len].iter().min().unwrap()
    );
}
