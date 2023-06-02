#[derive(Default, Debug)]
enum Player {
    #[default]
    SK,
    CY,
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i64 = buf.trim().parse().unwrap();
    let mut winners = vec![Default::default(), Player::SK, Player::CY];

    for i in 3..=5 {
        let winner = 'w: {
            if i == 4 {
                break 'w Player::SK;
            }

            if [1, 4]
                .iter()
                .filter(|&&power_4| power_4 < i)
                .any(|p| matches!(winners[i - p], Player::CY))
            {
                Player::SK
            } else {
                Player::CY
            }
        };

        winners.push(winner);
    }

    let idx = (n % 5) as usize;

    println!("{:?}", winners[if idx == 0 { 5 } else { idx }]);
}
