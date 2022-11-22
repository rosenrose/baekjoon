use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();

    let fibo_num = fibo(n);

    for i in fibo_num.iter().rev() {
        write!(stdout, "{i}").unwrap();
    }
}

fn fibo(n: i32) -> Vec<i32> {
    if n <= 1 {
        return Vec::from([n]);
    }

    let (mut a, mut b) = (Vec::from([1]), Vec::from([1]));

    for _ in 2..n {
        let next = add(&mut a, &mut b);
        (a, b) = (b, next);
    }

    b
}

fn add(a: &mut Vec<i32>, b: &mut Vec<i32>) -> Vec<i32> {
    let mut carry = 0;
    let mut sum: Vec<_> = (0..a.len().max(b.len()))
        .map(|i| {
            let temp = carry + a.get(i).unwrap_or(&0) + b.get(i).unwrap_or(&0);
            carry = temp / 10;

            temp % 10
        })
        .collect();

    if carry > 0 {
        sum.push(carry);
    }

    sum
}
