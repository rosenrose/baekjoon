use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [x, y, w, h] = [(); 4].map(|_| input.next().unwrap());
    let horizontal_min = x.min(w - x);
    let vertical_min = y.min(h - y);

    println!("{}", horizontal_min.min(vertical_min));
}
