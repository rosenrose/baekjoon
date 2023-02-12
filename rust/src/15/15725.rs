fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let Some((coef_x, _)) = buf.split_once('x') else {
        println!("0");
        return;
    };

    println!(
        "{}",
        match coef_x {
            "" => 1,
            "-" => -1,
            x => x.parse().unwrap(),
        }
    );
}
