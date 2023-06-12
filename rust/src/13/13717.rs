use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let (mut sum, mut max_count) = (0, 0);
    let counts: Vec<_> = (0..parse_int(input.next().unwrap()))
        .map(|_| {
            let [name, k, m] = [(); 3].map(|_| input.next().unwrap());
            let [k, mut m] = [k, m].map(parse_int);
            let mut count = 0;

            while m >= k {
                m -= k - 2;
                count += 1;
            }

            sum += count;
            max_count = count.max(max_count);

            (name, count)
        })
        .collect();

    let max_name = counts
        .iter()
        .find_map(|&(name, c)| (c == max_count).then_some(name))
        .unwrap();

    println!("{sum}\n{max_name}");
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
