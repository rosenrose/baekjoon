use std::collections::VecDeque;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();

    if n == 1 {
        println!("1");
        return;
    }

    let mut cards: VecDeque<i32> = (1..=n).collect();

    loop {
        cards.pop_front();

        if cards.len() == 1 {
            break;
        }

        let num = cards.pop_front().unwrap();
        cards.push_back(num);
    }

    println!("{}", cards[0]);
}
