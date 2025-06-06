use std::io;

const MAX: usize = 4;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let [_, k] = [(); 2].map(|_| input.next().unwrap().parse::<usize>().unwrap());
    let nums: Vec<_> = input.collect();
    let mut results = Vec::new();

    permutations(0, &mut [0; MAX][..k], &mut [false; 10], &nums, &mut results);

    results.sort();
    results.dedup();

    println!("{}", results.len());
}

fn permutations(
    depth: usize,
    selected: &mut [usize],
    visited: &mut [bool],
    nums: &[&str],
    results: &mut Vec<String>,
) {
    if depth == selected.len() {
        let num: Vec<_> = selected.iter().map(|&i| nums[i]).collect();
        results.push(num.join(""));

        return;
    }

    for i in 0..nums.len() {
        if visited[i] {
            continue;
        }

        visited[i] = true;
        selected[depth] = i;

        permutations(depth + 1, selected, visited, nums, results);

        visited[i] = false;
    }
}
