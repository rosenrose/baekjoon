use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();
    let [ah, require] = [(); 2].map(|_| input.next().unwrap());

    println!(
        "{}",
        if ah.len() >= require.len() {
            "go"
        } else {
            "no"
        }
    );
}
