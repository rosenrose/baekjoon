fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();

    println!("{}", domino_num(n));
}

fn domino_num(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 3;
    }
    // 2 * domino_num(n - 1) - domino_num(n - 2) + 3 * n

    let (mut a, mut b) = (0, 3);

    for i in 2..=n {
        (a, b) = (b, 2 * b - a + 3 * i);
    }

    b
}

/*
0
0 0
= 0

1
0 0
0 1 = 0 0 + 0 1
1 1 = 0 0 + 1 1
= 3

2
0 0
0 1
1 1

0 2 = 0 1 + 0 1
1 2 = 1 1 + 0 1
2 2 = 1 1 + 1 1
= 12

3
0 0
0 1
1 1

0 2
1 2
2 2

0 3 = 0 2 + 0 1
1 3 = 1 2 + 0 1
2 3 = 2 2 + 0 1
3 3 = 2 2 + 1 1
= 30

f(n) = f(n-1) + (f(n-1) - f(n-2) + 2(n-1)) + n+2
f(n) = 2(f(n-1)) - f(n-2) + 3n
*/
