use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<f64>);

    while let [Some(n), Some(b), Some(m)] = [(); 3].map(|_| input.next()) {
        let years = (m / n).log(1.0 + b / 100.0);

        println!("{}", years.ceil());
    }
}
