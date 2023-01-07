use std::io;

#[derive(Copy, Clone)]
enum Ops {
    Add = 0,
    Sub,
    Mul,
    Div,
}

const MAX: i32 = 1_000_000_000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let n = input.next().unwrap();
    let nums: Vec<_> = (0..n).map(|_| input.next().unwrap()).collect();
    let operators: Vec<_> = input
        .enumerate()
        .flat_map(|(i, count)| {
            (0..count).map(move |_| match i {
                0 => Ops::Add,
                1 => Ops::Sub,
                2 => Ops::Mul,
                _ => Ops::Div,
            })
        })
        .collect();

    let mut selected = Vec::new();
    let (min, max) = formula_min_max(0, &operators, &mut selected, &nums);

    println!("{max}\n{min}");
}

fn formula_min_max(
    depth: usize,
    operators: &Vec<Ops>,
    selected: &mut Vec<usize>,
    nums: &Vec<i32>,
) -> (i32, i32) {
    if depth == operators.len() {
        let result = selected.iter().enumerate().fold(nums[0], |acc, (i, &sel)| {
            let num = nums[i + 1];

            match operators[sel] {
                Ops::Add => acc + num,
                Ops::Sub => acc - num,
                Ops::Mul => acc * num,
                Ops::Div => acc / num,
            }
        });

        return (result, result);
    }

    let mut visited = [false; 4];

    operators
        .iter()
        .enumerate()
        .fold((MAX, -MAX), |(min, max), (i, &op)| {
            if selected.contains(&i) || visited[op as usize] {
                return (min, max);
            }

            visited[op as usize] = true;
            selected.push(i);

            let result = formula_min_max(depth + 1, operators, selected, nums);

            selected.pop();

            (result.0.min(min), result.1.max(max))
        })
}
