const EXP: i128 = 10_000_000_000_000_000_000_000_000_000_000_000_000;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i128 = buf.trim().parse().unwrap();
    let fibo_num = fibo(n);

    for (i, num) in fibo_num.iter().rev().enumerate() {
        if i == 0 {
            print!("{num}");
        } else {
            print!("{num:037}");
        }
    }
}

fn fibo(n: i128) -> Vec<i128> {
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

fn add(a: &mut Vec<i128>, b: &mut Vec<i128>) -> Vec<i128> {
    let mut carry = 0;
    let mut sum: Vec<_> = (0..a.len().max(b.len()))
        .map(|i| {
            let temp = carry + a.get(i).unwrap_or(&0) + b.get(i).unwrap_or(&0);
            carry = temp / EXP;

            temp % EXP
        })
        .collect();

    if carry > 0 {
        sum.push(carry);
    }

    sum
}
