fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let price: i32 = buf.trim().parse().unwrap();
    let mut charge = 1000 - price;

    let count = [500, 100, 50, 10, 5, 1].iter().fold(0, |acc, coin| {
        let count = acc + (charge / coin);
        charge %= coin;

        count
    });

    println!("{count}");
}
