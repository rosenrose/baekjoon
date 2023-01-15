use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let (_, m) = (input.next(), input.next().unwrap());

    let mut nums: Vec<_> = input.collect();
    nums.sort();

    permutation(&nums, m, &mut Vec::new(), &mut output);

    print!("{output}");
}

fn permutation(nums: &Vec<i32>, m: i32, selected: &mut Vec<i32>, output: &mut String) {
    if m == 0 {
        for num in selected {
            write!(output, "{num} ").unwrap();
        }
        writeln!(output, "").unwrap();

        return;
    }

    for &num in nums {
        if selected.contains(&num) {
            continue;
        }

        selected.push(num);

        permutation(nums, m - 1, selected, output);

        selected.pop();
    }
}
