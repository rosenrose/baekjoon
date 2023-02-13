fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    for sentence in buf.trim().split("What is") {
        if let Some((question, _)) = sentence.split_once('?') {
            println!("Forty-two is{question}.");
        }
    }
}
