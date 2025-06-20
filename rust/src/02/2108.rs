use std::io;

const MAX: usize = 500_000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let mut sum = 0;
    let mut counts = [0; 8001];
    let mut nums = [0; MAX];

    for (i, num) in input.enumerate() {
        sum += num;
        counts[(num + 4000) as usize] += 1;
        nums[i] = num;
    }

    nums[..n].sort_unstable();

    let (min, max, middle) = (nums[0], nums[n - 1], nums[n / 2]);
    let avg = (sum as f64 / n as f64).round() as i32;

    let max_count = counts.iter().max().unwrap();
    let mut mosts = [None; 2];

    for (i, count) in counts.iter().enumerate() {
        if count != max_count {
            continue;
        }

        if mosts[0].is_none() {
            mosts[0] = Some(i as i32 - 4000);
        } else {
            mosts[1] = Some(i as i32 - 4000);
            break;
        }
    }

    let most = mosts[1].unwrap_or(mosts[0].unwrap());

    for value in [avg, middle, most, max - min] {
        println!("{value}");
    }
}
