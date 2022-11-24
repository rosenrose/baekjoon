use std::collections::VecDeque;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut nums = buf.split_whitespace();
    let n = parse_bigint(nums.next().unwrap());
    let m = parse_bigint(nums.next().unwrap());
    let (q, r) = divide(&n, &m);

    print(&q);
    println!("");
    print(&r);
}

fn parse_bigint(input: &str) -> VecDeque<i8> {
    input
        .chars()
        .rev()
        .map(|c| c.to_digit(10).unwrap() as i8)
        .collect()
}

fn print(bigint: &VecDeque<i8>) {
    bigint.iter().rev().for_each(|num| {
        print!("{num}");
    })
}

fn is_zero(bigint: &VecDeque<i8>) -> bool {
    bigint.iter().all(|&i| i == 0)
}

fn is_less(a: &VecDeque<i8>, b: &VecDeque<i8>) -> bool {
    if a.len() == b.len() {
        a.iter().rev().lt(b.iter().rev())
    } else {
        a.len() < b.len()
    }
}

fn divide(a: &VecDeque<i8>, b: &VecDeque<i8>) -> (VecDeque<i8>, VecDeque<i8>) {
    let mut divisor_multiples = vec![b.clone()];

    for _ in 0..8 {
        let multiple = divisor_multiples.last().unwrap();
        divisor_multiples.push(add(multiple, &b));
    }

    let (mut dividend, mut quotient) = (VecDeque::new(), VecDeque::new());

    for i in (0..=a.len() - b.len()).rev() {
        if dividend.is_empty() {
            dividend = a.range(i..).copied().collect();
        } else {
            if is_zero(&dividend) {
                dividend.clear();
            }

            dividend.push_front(a[i]);
        }
        // println!("{dividend:?} {quotient:?}");
        if is_less(&dividend, b) {
            if i < a.len() - b.len() {
                quotient.push_front(0);
            }
            continue;
        }

        for (q, divisor) in divisor_multiples.iter().enumerate() {
            let diff = sub(&dividend, divisor);
            // println!("{dividend:?} {divisor:?} {diff:?}");

            if is_less(&diff, b) {
                quotient.push_front(q as i8 + 1);
                dividend = diff;
                break;
            }
        }
    }

    (quotient, dividend)
}

fn add(a: &VecDeque<i8>, b: &VecDeque<i8>) -> VecDeque<i8> {
    let mut carry = 0;
    let mut sum: VecDeque<_> = (0..a.len().max(b.len()))
        .map(|i| {
            let temp = carry + *a.get(i).unwrap_or(&0) + *b.get(i).unwrap_or(&0);
            carry = temp / 10;

            temp % 10
        })
        .collect();

    if carry > 0 {
        sum.push_back(carry);
    }

    sum
}

fn sub(a: &VecDeque<i8>, b: &VecDeque<i8>) -> VecDeque<i8> {
    let mut carry = 0;
    let mut diff: VecDeque<_> = (0..a.len().max(b.len()))
        .map(|i| {
            let temp = carry + *a.get(i).unwrap_or(&0) - *b.get(i).unwrap_or(&0);

            if temp < 0 {
                carry = -1;
                temp + 10
            } else {
                carry = 0;
                temp
            }
        })
        .collect();

    while diff.len() > 1 && *diff.back().unwrap() == 0 {
        diff.pop_back();
    }

    diff
}
