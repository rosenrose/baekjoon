use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for input in buf.lines().skip(1) {
        let input = input.as_bytes();
        let counts: Vec<_> = ["TTT", "TTH", "THT", "THH", "HTT", "HTH", "HHT", "HHH"]
            .iter()
            .map(|coin| input.windows(3).filter(|&w| w == coin.as_bytes()).count())
            .collect();

        for c in counts {
            print!("{c} ");
        }
        println!("");
    }
}
