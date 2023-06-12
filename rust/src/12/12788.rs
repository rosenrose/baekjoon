use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [_, m, k] = [(); 3].map(|_| input.next().unwrap());
    let mut needs = m * k;
    let mut pens: Vec<_> = input.collect();

    pens.sort();

    for (i, pen) in pens.iter().rev().enumerate() {
        needs -= pen;

        if needs <= 0 {
            println!("{}", i + 1);
            return;
        }
    }

    println!("STRESS");
}
