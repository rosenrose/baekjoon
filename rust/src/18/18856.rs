fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();

    print!("{n}\n1 2 ");

    for i in 3..n {
        print!("{i} ");
    }

    println!("{}", (n..).find(|&n| is_prime(n)).unwrap());
}

fn is_prime(num: i32) -> bool {
    if num == 1 {
        return false;
    }

    (2..).take_while(|i| i * i <= num).all(|i| num % i != 0)
}
