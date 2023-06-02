#[derive(Default, Debug)]
enum Player {
    #[default]
    SK,
    CY,
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();
    let mut winners = vec![
        Default::default(),
        Player::CY,
        Player::SK,
        Player::CY,
        Player::SK,
    ];

    for i in 5..=n {
        winners.push(
            if [i - 1, i - 3, i - 4]
                .iter()
                .any(|&i| matches!(winners[i], Player::CY))
            {
                Player::SK
            } else {
                Player::CY
            },
        );
    }
    // println!("{winners:?}");
    println!("{:?}", winners[n]);
}
