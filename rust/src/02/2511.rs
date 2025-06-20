use std::cmp::Ordering::{Equal, Greater, Less};
use std::io;

enum Player {
    A,
    B,
}

const CARDS: usize = 10;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let mut a_cards = [0; CARDS];
    let mut b_cards = [0; CARDS];

    for (i, num) in input.by_ref().take(CARDS).enumerate() {
        a_cards[i] = num;
    }

    for (i, num) in input.enumerate() {
        b_cards[i] = num;
    }

    let mut last_winner = None;
    let (a_score, b_score) = a_cards
        .iter()
        .zip(b_cards)
        .fold((0, 0), |(a, b), (a_num, b_num)| match a_num.cmp(&b_num) {
            Greater => {
                last_winner = Some(Player::A);
                (a + 3, b)
            }
            Equal => (a + 1, b + 1),
            Less => {
                last_winner = Some(Player::B);
                (a, b + 3)
            }
        });

    println!("{a_score} {b_score}");
    println!(
        "{}",
        match a_score.cmp(&b_score) {
            Greater => "A",
            Less => "B",
            Equal => match last_winner {
                Some(Player::A) => "A",
                Some(Player::B) => "B",
                None => "D",
            },
        }
    )
}
