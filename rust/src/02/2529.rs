use std::io;

#[derive(Default)]
enum Ops {
    #[default]
    Less,
    Greater,
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let k: usize = input.next().unwrap().parse().unwrap();
    let operators: Vec<_> = input
        .map(|op| match op {
            "<" => Ops::Less,
            ">" => Ops::Greater,
            _ => Default::default(),
        })
        .collect();

    let nums: Vec<_> = ('0'..='9').collect();
    let (min, max) = formula_min_max(&nums, &operators, k + 1, &mut Vec::new());

    println!("{max:0digits$}\n{min:0digits$}", digits = k + 1);
}

fn formula_min_max(
    nums: &Vec<char>,
    operators: &Vec<Ops>,
    m: usize,
    selected: &mut Vec<char>,
) -> (i64, i64) {
    if m == 0 {
        let result = String::from_iter(selected.to_vec()).parse().unwrap();

        return (result, result);
    }

    nums.iter().fold((9876543210, 1), |(min, max), &num| {
        if selected.contains(&num) {
            return (min, max);
        }

        selected.push(num);

        let result = if (1..selected.len()).all(|i| match operators[i - 1] {
            Ops::Less => selected[i - 1] < selected[i],
            Ops::Greater => selected[i - 1] > selected[i],
        }) {
            formula_min_max(nums, operators, m - 1, selected)
        } else {
            (min, max)
        };

        selected.pop();

        (result.0.min(min), result.1.max(max))
    })
}
