use std::io;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Player {
    O = 1,
    X,
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let mut player = if input() == 1 { Player::O } else { Player::X };
    let mut board = [[None; 3]; 3];

    for (r, c) in (0..9).map(|_| (input() - 1, input() - 1)) {
        board[r][c] = Some(player);

        if let Some(winner) = is_gameover(&board) {
            println!("{}", winner as i32);
            return;
        }

        player = match player {
            Player::O => Player::X,
            Player::X => Player::O,
        }
    }

    println!("0");
}

fn is_gameover(board: &[[Option<Player>; 3]; 3]) -> Option<Player> {
    for player in [Some(Player::O), Some(Player::X)] {
        if (0..3).any(|r| (0..3).all(|c| board[r][c] == player)) {
            return player;
        }
        if (0..3).any(|c| (0..3).all(|r| board[r][c] == player)) {
            return player;
        }

        if (0..3).all(|i| board[i][i] == player) || (0..3).all(|i| board[i][2 - i] == player) {
            return player;
        }
    }

    None
}
