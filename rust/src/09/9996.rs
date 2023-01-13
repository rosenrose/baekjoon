use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();
    input.next();

    let mut pattern = input.next().unwrap().split('*');
    let (prefix, suffix) = (pattern.next().unwrap(), pattern.next().unwrap());

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
