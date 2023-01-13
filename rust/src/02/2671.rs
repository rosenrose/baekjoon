fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    println!(
        "{}",
        if Regex::new("^(100+1+|01)+$", false)
            .find(buf.trim())
            .is_some()
        {
            "SUBMARINE"
        } else {
            "NOISE"
        }
    );
}
