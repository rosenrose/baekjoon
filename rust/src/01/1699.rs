fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();
    let mut memo: Vec<_> = (0..=n as i32).collect();

    for i in (1..).take_while(|i| i * i <= n) {
        memo[i * i] = 1;
    }

    println!("{}", square_count(n, &mut memo));
}

fn square_count(num: usize, memo: &mut Vec<i32>) -> i32 {
    for i in 2..=num {
        for j in (1..).take_while(|j| j * j <= i) {
            memo[i] = memo[i].min(1 + memo[i - j * j]);
        }
    }

    memo[num]
}
