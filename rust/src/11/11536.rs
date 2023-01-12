use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let names: Vec<_> = buf.lines().skip(1).collect();

    if (1..names.len()).all(|i| names[i - 1] < names[i]) {
        println!("INCREASING");
        return;
    }

    if (1..names.len()).all(|i| names[i - 1] > names[i]) {
        println!("DECREASING");
        return;
    }

    println!("NEITHER");
}
