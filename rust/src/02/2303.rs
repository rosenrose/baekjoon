use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    const CARDS: usize = 5;

    let idx = (1..=input.next().unwrap())
        .max_by_key(|_| {
            let nums: Vec<_> = input.by_ref().take(CARDS).collect();
            let mut max_sum = 0;

            for a in 0..CARDS - 2 {
                for b in a + 1..CARDS - 1 {
                    for c in b + 1..CARDS {
                        max_sum = ((nums[a] + nums[b] + nums[c]) % 10).max(max_sum);
                    }
                }
            }

            max_sum
        })
        .unwrap();

    println!("{idx}");
}
