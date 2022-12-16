use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let (n, k) = (input.next().unwrap(), input.next().unwrap() as usize);
    let mut cache = vec![0; k + 1];

    for i in 0..n {
        let (w, v) = (input.next().unwrap() as usize, input.next().unwrap());

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
