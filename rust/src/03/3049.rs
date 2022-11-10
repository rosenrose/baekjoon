fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();
    let mut an = vec![0, 0];
    let mut bn = vec![0, 0];
    let mut cn = vec![0, 0];

    for i in 2..=n {
        an.push(i - 1);
        bn.push(bn[i - 1] + an[i - 1]);
        cn.push(cn[i - 1] + bn[i - 1]);
    }
    // println!("{an:?}\n{bn:?}\n{cn:?}");

    let points = cn[n] * n / 4;

    println!("{points}");
}
