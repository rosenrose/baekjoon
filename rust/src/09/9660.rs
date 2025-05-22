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

    let mut winners = [Default::default(); 8];
    winners[1..5].copy_from_slice(&[Player::SK, Player::CY, Player::SK, Player::SK]);

    for i in 5..=7 {
        winners[i] = if [i - 1, i - 3, i - 4]
            .iter()
            .any(|&i| winners[i] == Player::CY)
        {
            Player::SK
        } else {
            Player::CY
        }
    }

    let idx = n % 7;

    println!("{:?}", winners[if idx == 0 { 7 } else { idx }]);
}
