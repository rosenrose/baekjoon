use std::io;

#[derive(Copy, Clone, Default, PartialEq, Debug)]
enum Player {
    #[default]
    SK,
    CY,
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    let mut winners = [Default::default(); 6];
    winners[1] = Player::SK;
    winners[2] = Player::CY;

    for i in 3..=5 {
        let winner = 'w: {
            if i == 4 {
                break 'w Player::SK;
            }

            if [1, 4]
                .iter()
                .filter(|&&power_4| power_4 < i)
                .any(|p| winners[i - p] == Player::CY)
            {
                Player::SK
            } else {
                Player::CY
            }
        };

        winners[i] = winner;
    }

    let idx = n % 5;

    println!("{:?}", winners[if idx == 0 { 5 } else { idx }]);
}
