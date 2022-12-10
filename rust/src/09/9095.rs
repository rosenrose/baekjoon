use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf.lines().map(|s| s.parse::<i32>().unwrap());

    for num in input.skip(1) {
        println!("{}", permutation_123(num));
    }
}

fn permutation_123(num: i32) -> i32 {
    if num <= 2 {
        return num;
    }
    if num == 3 {
        return 4;
    }
    //(num - 3..=num - 1).map(|n| permutation_123(n)).sum()
    let (mut a, mut b, mut c) = (1, 2, 4);

    for _ in 4..=num {
        (a, b, c) = (b, c, a + b + c);
    }

    c
}
