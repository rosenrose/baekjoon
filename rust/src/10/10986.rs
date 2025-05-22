use std::io;

const MAX: usize = 1_000_000 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [_, m] = [(); 2].map(|_| input.next().unwrap());
    let mut rem_accum = [0; MAX];
    let mut rem_count = [0; 1000];

    for (i, num) in input.enumerate() {
        rem_accum[i + 1] = (rem_accum[i] + num) % m;
        rem_count[rem_accum[i + 1] as usize] += 1;
    }

    let count = rem_count[0]
        + rem_count[..m as usize]
            .iter()
            .filter_map(|&num| (num > 0).then(|| num * (num - 1) / 2))
            .sum::<usize>();

    println!("{count}");
}
