use std::io;

#[derive(Copy, Clone, Default, PartialEq, Debug)]
enum Player {
    #[default]
    SK,
    CY,
}

const MAX: usize = 1000 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    let mut winners = [Default::default(); MAX];
    winners[1..5].copy_from_slice(&[Player::SK, Player::CY, Player::SK, Player::SK]);

    for i in 5..=n {
        winners[i] = if [i - 1, i - 3, i - 4]
            .iter()
            .any(|&i| winners[i] == Player::CY)
        {
            Player::SK
        } else {
            Player::CY
        };
    }
    // println!("{winners:?}");
    println!("{:?}", winners[n]);
}
