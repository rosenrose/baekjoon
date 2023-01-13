use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for input in buf.lines().take_while(|&input| input != "#") {
        let (idx, _) = input
            .match_indices(['a', 'e', 'i', 'o', 'u'])
            .next()
            .unwrap_or((0, ""));

        println!("{}{}ay", &input[idx..], &input[..idx]);
    }
}
