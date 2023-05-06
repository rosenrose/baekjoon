fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut cards = [[false; 13]; 4];

    for card in buf.trim().as_bytes().chunks(3) {
        let shape = match card[0] as char {
            'P' => 0_usize,
            'K' => 1,
            'H' => 2,
            'T' => 3,
            _ => unreachable!(),
        };
        let number = ((card[1] - '0' as u8) * 10 + card[2] - '0' as u8) as usize - 1;

        if cards[shape][number] {
            println!("GRESKA");
            return;
        }

        cards[shape][number] = true;
    }

    for card in cards {
        print!("{} ", card.iter().filter(|&b| !b).count());
    }
}
