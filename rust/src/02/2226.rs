const EXP: i128 = 10_000_000_000_000_000_000_000_000_000_000_000_000;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i128 = buf.trim().parse().unwrap();
    // let bin = (0..n - 1).fold(1.to_string(), |acc, _| {
    //     acc.chars()
    //         .map(|c| if c == '0' { "10" } else { "01" })
    //         .collect()
    // });
    // println!("{bin}");

    let count = (2..=n).fold(vec![0], |acc, num| {
        if num % 2 == 0 {
            add(mul(acc, 2), 1)
        } else {
            sub(mul(acc, 2), 1)
        }
    });

    for (i, num) in count.iter().rev().enumerate() {
        if i == 0 {
            print!("{num}");
        } else {
            print!("{num:037}");
        }
    }
}

fn add(mut a: Vec<i128>, b: i128) -> Vec<i128> {
    a[0] += b;
    let mut carry = a[0] / EXP;

    for i in 1..a.len() {
        if carry == 0 {
            break;
        }

        a[i - 1] %= EXP;
        a[i] += carry;
        carry = a[i] / EXP;
    }

    if carry > 0 {
        a.push(carry);
    }

    a
}

fn sub(mut a: Vec<i128>, b: i128) -> Vec<i128> {
    a[0] -= b;
    let mut carry = if a[0] < 0 { -1 } else { 0 };

    for i in 1..a.len() {
        if carry == 0 {
            break;
        }

        a[i - 1] += EXP;
        a[i] -= carry;
        carry = if a[i] < 0 { -1 } else { 0 };
    }

    while a.len() > 0 && *a.last().unwrap() == 0 {
        a.pop();
    }

    a
}

fn mul(a: Vec<i128>, b: i128) -> Vec<i128> {
    let mut carry = 0;
    let mut result: Vec<_> = a
        .iter()
        .map(|num| {
            let temp = carry + num * b;
            carry = temp / EXP;

            temp % EXP
        })
        .collect();

    if carry > 0 {
        result.push(carry);
    }

    result
}
