use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let sum: i32 = format!("9780921418{}", buf.replace(['\r', '\n'], ""))
        .char_indices()
        .map(|(i, c)| (c as i32 - '0' as i32) * if i % 2 == 0 { 1 } else { 3 })
        .sum();

    println!("The 1-3-sum is {sum}");
}
