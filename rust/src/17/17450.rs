use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let (snack, _) = ['S', 'N', 'U']
        .iter()
        .map(|s| {
            let [price, weight] = [(); 2].map(|_| input.next().unwrap() * 10);

            (
                s,
                weight as f64 / if price >= 5000 { price - 500 } else { price } as f64,
            )
        })
        .max_by(|(_, e1), (_, e2)| e1.total_cmp(&e2))
        .unwrap();

    println!("{snack}");
}
