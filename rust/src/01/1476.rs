use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let [e, s, m] = [(); 3].map(|_| input.next().unwrap());
    let mut year = s;

    loop {
        if (e % 15 == year % 15) && (m % 19 == year % 19) {
            println!("{year}");
            return;
        }

        year += 28;
    }
}
