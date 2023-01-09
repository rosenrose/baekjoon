use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let (snack, _) = ['S', 'N', 'U']
        .iter()
        .map(|s| {
            let (price, weight) = (input.next().unwrap() * 10, input.next().unwrap() * 10);

            (
                s,
                weight as f32 / if price >= 5000 { price - 500 } else { price } as f32,
            )
        })
        .max_by(|(_, e1), (_, e2)| e1.total_cmp(&e2))
        .unwrap();

    println!("{snack}");
}
