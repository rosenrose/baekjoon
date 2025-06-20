use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [x, y] = [(); 2].map(|_| input.next().unwrap());
    // let gcd = get_gcd(x, y);
    // let (x, y) = (x / gcd, y / gcd);
    // println!("{}", (x + y - 1) * gcd);

    println!("{}", x + y - get_gcd(x, y));
}

fn get_gcd(mut a: i32, mut b: i32) -> i32 {
    loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    }
}
