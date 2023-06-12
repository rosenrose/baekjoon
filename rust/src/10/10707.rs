use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<i32>);

    let [a, b, c, d, p] = [(); 5].map(|_| input.next().unwrap());
    let x_bill = a * p;
    let mut y_bill = b;

    if p > c {
        y_bill += (p - c) * d;
    }

    println!("{}", x_bill.min(y_bill));
}
