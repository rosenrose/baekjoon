use std::io;

const CARDS: usize = 5;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let idx = (1..=input.next().unwrap())
        .max_by_key(|_| {
            let mut nums = [0; CARDS];

            for (i, num) in input.by_ref().take(nums.len()).enumerate() {
                nums[i] = num;
            }

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
