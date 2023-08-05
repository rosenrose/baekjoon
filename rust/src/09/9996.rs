use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let [_, pattern] = [(); 2].map(|_| input.next().unwrap());
    let (prefix, suffix) = pattern.split_once('*').unwrap();

    let regex = Regex::new(&format!("^{prefix}.*{suffix}$"), false);

    for file_name in input {
        println!(
            "{}",
            if regex.find(file_name).is_some() {
                "DA"
            } else {
                "NE"
            }
        );
    }
}
