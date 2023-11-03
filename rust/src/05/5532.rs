use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<u32>);

    let [l, a, b, c, d] = [(); 5].map(|_| input.next().unwrap());
    let lang_count = a.div_ceil(c);
    let math_count = b.div_ceil(d);

    println!("{}", l - lang_count.max(math_count));
}
