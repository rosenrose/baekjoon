fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();
    let mut memo = [0; 50_000 + 1];

    for i in (1..).take_while(|i| i * i <= n) {
        memo[i * i] = 1;
    }

    println!("{}", square_count(n, &mut memo));
}

fn square_count(num: usize, memo: &mut [u8]) -> u8 {
    for i in 2..=num {
        if memo[i] != 0 {
            continue;
        }

        memo[i] = (1..)
            .map_while(|j| {
                let square = j * j;
                (square < i).then(|| 1 + memo[i - square])
            })
            .min()
            .unwrap();
    }

    memo[num]
}
