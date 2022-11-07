fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [m, n] = parse_int_vec(&buf)[..] {
        let mut nums: Vec<_> = (m..=n)
            .map(|num| {
                let eng: String = num
                    .to_string()
                    .chars()
                    .map(|c| match c {
                        '0' => "z",
                        '1' => "o",
                        '2' => "tw",
                        '3' => "th",
                        '4' => "fo",
                        '5' => "fi",
                        '6' => "si",
                        '7' => "se",
                        '8' => "e",
                        '9' => "n",
                        _ => "",
                    })
                    .collect();

                (num, eng)
            })
            .collect();

        nums.sort_by(|(_, eng1), (_, eng2)| eng1.cmp(eng2));

        for chunk in nums.chunks(10) {
            for (num, _) in chunk {
                print!("{num} ");
            }
            println!("");
        }
    }
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
