#[derive(Default, PartialEq, Debug)]
enum Winner {
    #[default]
    SK,
    CY,
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i64 = buf.trim().parse().unwrap();
    let mut winner = vec![
        Default::default(),
        Winner::SK,
        Winner::CY,
        Winner::SK,
        Winner::SK,
    ];

    for i in 5..=7 {
        winner.push(
            if [i - 1, i - 3, i - 4]
                .iter()
                .any(|&i| winner[i] == Winner::CY)
            {
                Winner::SK
            } else {
                Winner::CY
            },
        );
    }

    let idx = (n % 7) as usize;

    println!("{:?}", winner[if idx == 0 { 7 } else { idx }]);
}
