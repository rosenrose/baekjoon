use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let (n, k) = (input(), input() as usize);
    let mut cache = vec![0; k + 1];

    for i in 0..n {
        let (w, v) = (input() as usize, input());

        if i == 0 {
            for j in w..=k {
                cache[j] = v;
            }

            continue;
        }

        cache = cache
            .iter()
            .enumerate()
            .map(|(j, &max_val)| {
                if j < w {
                    max_val
                } else {
                    max_val.max(v + cache[j - w])
                }
            })
            .collect();
    }

    println!("{}", cache[k]);
}
