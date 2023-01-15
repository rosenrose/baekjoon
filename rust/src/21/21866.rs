fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let scores: Vec<i32> = buf.split_whitespace().flat_map(str::parse).collect();
    const MAX: [i32; 9] = [100, 100, 200, 200, 300, 300, 400, 400, 500];

    if scores.iter().zip(MAX).any(|(&score, max)| score > max) {
        println!("hacker");
        return;
    }

    println!(
        "{}",
        if scores.iter().sum::<i32>() < 100 {
            "none"
        } else {
            "draw"
        }
    );
}
