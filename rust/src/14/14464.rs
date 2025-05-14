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
    let (mut min, mut max) = (0, chickens.len() - 1);
    let mut count = 0;

    for cow in cows {
        let Some(chicken_idx) = binary_search(&chickens, cow, (min as i32, max as i32), &visited)
        else {
            continue;
        };

        visited[chicken_idx] = true;
        count += 1;

        while min < chickens.len() && visited[min] {
            min += 1;
        }
        while max > 0 && visited[max] {
            max -= 1;
        }
    }

    println!("{count}");
}

fn binary_search(
    chickens: &[i32],
    cow: (i32, i32),
    (mut lo, mut hi): (i32, i32),
    visited: &[bool],
) -> Option<usize> {
    let (start, end) = cow;
    let mut result = None;

    while lo <= hi {
        let mid = lo + ((hi - lo) >> 1);
        let chicken = chickens[mid as usize];

        if chicken < start {
            lo = mid + 1;
        } else if start <= chicken && chicken <= end {
            if !visited[mid as usize] {
                result = Some(mid as usize);
            }

            return binary_search(chickens, cow, (lo, mid - 1), visited)
                .or(result)
                .or_else(|| binary_search(chickens, cow, (mid + 1, hi), visited));
        } else if end < chicken {
            hi = mid - 1;
        }
    }

    None
}
