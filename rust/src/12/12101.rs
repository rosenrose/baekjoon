fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n, k] = parse_int_vec(&buf)[..] else { return };

    let mut perms = Vec::new();
    permutations(&mut Vec::new(), n, &mut perms);
    perms.sort();
    // println!("{perms:?}");
    let Some(result) = perms.get(k as usize - 1) else {
        println!("-1");
        return;
    };

    println!(
        "{}",
        format!("{:?}", result)
            .replace(['[', ']'], "")
            .replace(", ", "+")
    );
}

fn permutations(selected: &mut Vec<i32>, n: i32, result: &mut Vec<Vec<i32>>) {
    let sum: i32 = selected.iter().sum();

    if sum == n {
        result.push(selected.clone());
        return;
    }

    for num in [1, 2, 3] {
        if sum + num > n {
            continue;
        }

        selected.push(num);

        permutations(selected, n, result);

        selected.pop();
    }
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
