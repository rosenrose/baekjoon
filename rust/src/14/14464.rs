use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let (c, n) = (input(), input());
    let mut chickens: Vec<_> = (0..c).map(|_| input()).collect();
    let mut cows: Vec<_> = (0..n).map(|_| (input(), input())).collect();

    chickens.sort_unstable();
    cows.sort_unstable_by_key(|&(a, b)| (b, a));
    // println!("{chickens:?}\n{cows:?}");
    let mut visited = [false; 20_000 + 1];
    let mut count = 0;

    for (start, end) in cows {
        let Some(chicken_idx) = binary_search(&chickens, start, end, &visited) else {
            continue;
        };

        visited[chicken_idx] = true;
        count += 1;
    }

    println!("{count}");
}

fn binary_search(chickens: &[i32], start: i32, end: i32, visited: &[bool]) -> Option<usize> {
    let (mut lo, mut hi) = (0, chickens.len() as i32 - 1);
    let mut result = None;

    while lo <= hi {
        let mid = (lo + ((hi - lo) >> 1)) as usize;

        if chickens[mid] < start {
            lo = mid as i32 + 1;
        } else if start <= chickens[mid] && chickens[mid] <= end {
            result = Some(mid);
            hi = mid as i32 - 1;
        } else if end < chickens[mid] {
            hi = mid as i32 - 1;
        }
    }

    let Some(mut result) = result else {
        return None;
    };

    while result < chickens.len() && chickens[result] <= end {
        if !visited[result] {
            return Some(result);
        }

        result += 1;
    }

    None
}
