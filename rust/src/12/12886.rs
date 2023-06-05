fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [a, b, c] = parse_int_vec(&buf)[..] else { return };
    let sum = a + b + c;

    const MAX: usize = 500 * 3;
    let mut visited = vec![vec![false; MAX + 1]; MAX + 1];
    visited[a as usize][b as usize] = true;

    let mut stack = vec![(a, b)];

    while let Some((a, b)) = stack.pop() {
        let nums = [a, b, sum - (a + b)];

        if nums[0] == nums[1] && nums[1] == nums[2] {
            println!("1");
            return;
        }

        let adjacents = [(0, 1), (0, 2), (1, 2)].iter().filter_map(|&(i, j)| {
            (nums[i] != nums[j]).then(|| {
                let x = nums[i].min(nums[j]);
                let mut adj = nums;

                if nums[i] < nums[j] {
                    adj[i] += x;
                    adj[j] -= x;
                } else {
                    adj[j] += x;
                    adj[i] -= x;
                }

                (adj[0], adj[1])
            })
        });

        for adj in adjacents {
            if visited[adj.0 as usize][adj.1 as usize] {
                continue;
            }

            visited[adj.0 as usize][adj.1 as usize] = true;
            stack.push(adj);
        }
    }

    println!("0");
}

fn parse_int_vec(buf: &str) -> Vec<i16> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
