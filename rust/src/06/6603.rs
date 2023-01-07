use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    while let Some(k) = input.next() {
        if k == 0 {
            break;
        }

        let nums: Vec<_> = (0..k).map(|_| input.next().unwrap()).collect();
        let mut selected = Vec::new();

        combination(&nums, 6, 0, &mut selected);

        println!("");
    }
}

fn combination(nums: &Vec<i32>, m: i32, start: usize, selected: &mut Vec<i32>) {
    if m == 0 {
        for num in selected {
            print!("{num} ");
        }
        println!("");

        return;
    }

    for i in start..nums.len() - (m as usize - 1) {
        if selected.contains(&nums[i]) {
            continue;
        }

        selected.push(nums[i]);

        combination(nums, m - 1, i + 1, selected);

        selected.pop();
    }
}
