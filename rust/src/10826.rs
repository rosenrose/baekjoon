use std::collections::VecDeque;
use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();

    let fibo_num = fibo(n);

    for i in fibo_num {
        write!(stdout, "{i}").unwrap();
    }
}

fn fibo(n: i32) -> VecDeque<i32> {
    if n <= 1 {
        return VecDeque::from([n]);
    }

    let (mut a, mut b) = (VecDeque::from([1]), VecDeque::from([1]));

    for _ in 2..n {
        let next = add_by_array(&mut a, &mut b);
        (a, b) = (b, next);
    }

    b
}

fn add_by_array(a: &mut VecDeque<i32>, b: &mut VecDeque<i32>) -> VecDeque<i32> {
    let longer = a.len().max(b.len());

    while a.len() < longer {
        a.push_front(0);
    }
    while b.len() < longer {
        b.push_front(0);
    }

    let mut sum: VecDeque<i32> = a.iter().zip(b.iter()).map(|(a, b)| a + b).collect();

    for i in (1..longer).rev() {
        if sum[i] < 10 {
            continue;
        }

        sum[i - 1] += sum[i] / 10;
        sum[i] %= 10;
    }

    while sum[0] >= 10 {
        sum.push_front(sum[0] / 10);
        sum[1] %= 10;
    }

    sum
}
