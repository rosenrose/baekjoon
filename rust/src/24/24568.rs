use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<i32>);

    let [regular, small] = [(); 2].map(|_| input.next().unwrap());

    println!("{}", regular * 8 + small * 3 - 28);
}
