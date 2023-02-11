use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let (n, _) = (input.next().unwrap(), input.next());
    let k: Vec<_> = input.collect();

    let num = (1..=n.to_string().len())
        .flat_map(|i| product(&k, i, &mut Vec::new()))
        // .inspect(|i| print!("{i} "))
        .filter(|&num| num <= n)
        .max()
        .unwrap();

    println!("{num}");
}

fn product(nums: &Vec<i64>, m: usize, selected: &mut Vec<i64>) -> Vec<i64> {
    if m == 0 {
        return vec![format!("{selected:?}")
            .replace(['[', ']', ',', ' '], "")
            .parse()
            .unwrap()];
    }

    let mut result = Vec::new();

    for &num in nums {
        selected.push(num);

        result.extend(product(nums, m - 1, selected));

        selected.pop();
    }

    result
}
