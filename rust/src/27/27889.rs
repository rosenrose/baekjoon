fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    println!(
        "{}",
        match buf.trim() {
            "NLCS" => "North London Collegiate School",
            "BHA" => "Branksome Hall Asia",
            "KIS" => "Korea International School",
            "SJA" => "St. Johnsbury Academy",
            _ => Default::default(),
        }
    );
}
