use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let (n, m) = (input.next().unwrap() as usize, input.next().unwrap());
    let mut rem_accum = vec![0; n + 1];
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
