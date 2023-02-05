use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<i32>);

    for num in input.skip(1) {
        println!(
            "{}",
            if (num * num).to_string().ends_with(&num.to_string()) {
                "YES"
            } else {
                "NO"
            }
        );
    }
}
