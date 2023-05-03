use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let (_, k) = (
        input.next(),
        input.next().unwrap().parse::<usize>().unwrap(),
    );
    let nums: Vec<_> = input.collect();
    let mut results = Vec::new();

    permutations(0, &mut vec![0; k], &nums, &mut results);

    results.sort();
    results.dedup();

    println!("{}", results.len());
}

fn permutations(depth: usize, selected: &mut Vec<usize>, nums: &[&str], results: &mut Vec<String>) {
    if depth == selected.len() {
        let num: Vec<_> = selected.iter().map(|&i| nums[i]).collect();
        results.push(num.join(""));

        return;
    }

    for i in 0..nums.len() {
        if selected[..depth].contains(&i) {
            continue;
        }

        selected[depth] = i;
        permutations(depth + 1, selected, nums, results);
    }
}
