use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    for _ in 0..parse_int(input()) {
        match parse_int(input()) {
            1 => {
                let num = u64::from_str_radix(
                    &input()
                        .split('.')
                        .map(|s| format!("{:08b}", parse_int(s)))
                        .collect::<String>(),
                    2,
                )
                .unwrap();

                println!("{num}");
            }
            2 => {
                let ipv8: Vec<_> = format!("{:064b}", parse_int(input()))
                    .as_bytes()
                    .chunks(8)
                    .map(|chunk| {
                        chunk
                            .iter()
                            .rev()
                            .enumerate()
                            .map(|(i, &c)| if c as char == '1' { 1 << i } else { 0 })
                            .sum::<u8>()
                            .to_string()
                    })
                    .collect();

                println!("{}", ipv8.join("."));
            }
            _ => (),
        }
    }
}

fn parse_int(buf: &str) -> u64 {
    buf.parse().unwrap()
}
