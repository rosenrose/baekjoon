use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<u32>);

    for n in input {
        println!("{}", cantor(n));
    }
}

fn cantor(n: u32) -> String {
    if n == 0 {
        return "-".to_owned();
    }

    let inner = cantor(n - 1);
    let blank = " ".repeat(3_usize.pow(n - 1));

    format!("{inner}{blank}{inner}")
}
