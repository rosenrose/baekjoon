fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut input = buf.split_whitespace().map(str::as_bytes);
    let [a, b] = [(); 2].map(|_| input.next().unwrap());

    let min_diff = b
        .windows(a.len())
        .map(|window| a.iter().zip(window).filter(|(a, b)| a != b).count())
        .min()
        .unwrap();

    println!("{min_diff}");
}
