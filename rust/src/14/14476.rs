use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let nums: Vec<_> = input.skip(1).collect();
    let gcd_accum_left = nums.iter().fold(vec![nums[0]], |mut acc, num| {
        acc.push(get_gcd(*acc.last().unwrap(), *num));
        acc
    });
    let gcd_accum_right = nums
        .iter()
        .rev()
        .fold(vec![*nums.last().unwrap()], |mut acc, num| {
            acc.push(get_gcd(*acc.last().unwrap(), *num));
            acc
        });
    // println!("{gcd_accum_left:?} {gcd_accum_right:?}");
    let (mut k, mut max_gcd) = (0, 0);

    for (i, &num) in nums.iter().enumerate() {
        let gcd = match i {
            0 => *gcd_accum_right.iter().nth_back(1).unwrap(),
            i if i == nums.len() - 1 => *gcd_accum_left.iter().nth_back(1).unwrap(),
            _ => get_gcd(gcd_accum_left[i], gcd_accum_right[nums.len() - i - 1]),
        };

        if num % gcd == 0 {
            continue;
        }

        if gcd > max_gcd {
            max_gcd = gcd;
            k = num;
        }
    }

    if k == 0 {
        println!("-1");
        return;
    }

    println!("{max_gcd} {k}");
}

fn get_gcd(mut a: i32, mut b: i32) -> i32 {
    loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    }
}
