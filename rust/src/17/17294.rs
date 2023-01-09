fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let digits: Vec<_> = buf.trim().chars().map(|c| c as i8).collect();

    if digits.len() < 3 {
        println!("◝(⑅•ᴗ•⑅)◜..°♡ 뀌요미!!");
        return;
    }

    let diff = digits[0] - digits[1];

    println!(
        "{}",
        if (2..digits.len()).all(|i| digits[i - 1] - digits[i] == diff) {
            "◝(⑅•ᴗ•⑅)◜..°♡ 뀌요미!!"
        } else {
            "흥칫뿡!! <(￣ ﹌ ￣)>"
        }
    );
}
