use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input: Vec<_> = buf.lines().collect();

    const NUMS: [[&str; 5]; 10] = [
        ["***", "* *", "* *", "* *", "***"],
        ["  *", "  *", "  *", "  *", "  *"],
        ["***", "  *", "***", "*  ", "***"],
        ["***", "  *", "***", "  *", "***"],
        ["* *", "* *", "***", "  *", "  *"],
        ["***", "*  ", "***", "  *", "***"],
        ["***", "*  ", "***", "* *", "***"],
        ["***", "  *", "  *", "  *", "  *"],
        ["***", "* *", "***", "* *", "***"],
        ["***", "* *", "***", "  *", "***"],
    ];
    const BOOM: &str = "BOOM!!";

    let count = (input[0].len() + 1) / 4;
    let mut numbers = Vec::new();

    for i in 0..count {
        let start = i * 4;
        let end = start + 3;

        let Some(idx) = NUMS.iter().position(|&num| {
            num == [
                &input[0][start..end],
                &input[1][start..end],
                &input[2][start..end],
                &input[3][start..end],
                &input[4][start..end],
            ]
        }) else {
            println!("{BOOM}");
            return;
        };

        numbers.push(idx);
    }

    let sum: usize = numbers.iter().sum();
    let last = numbers.last().unwrap();

    println!(
        "{}",
        if sum % 3 == 0 && last % 2 == 0 {
            "BEER!!"
        } else {
            BOOM
        }
    );
}
