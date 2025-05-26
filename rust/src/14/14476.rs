use std::io;

const MAX: usize = 1_000_000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let mut nums = [0; MAX];

    for (i, num) in input.enumerate() {
        nums[i] = num;
    }

    let (mut gcd_accum_left, mut gcd_accum_right) = ([0; MAX + 1], [0; MAX + 1]);
    gcd_accum_left[0] = nums[0];
    gcd_accum_right[0] = nums[n - 1];
    let mut gcd_accum_len = 1;

    for i in 0..n {
        gcd_accum_left[gcd_accum_len] = get_gcd(gcd_accum_left[i], nums[i]);
        gcd_accum_right[gcd_accum_len] = get_gcd(gcd_accum_right[i], nums[n - 1 - i]);
        gcd_accum_len += 1;
    }
    // println!("{gcd_accum_left:?} {gcd_accum_right:?}");
    let (mut k, mut max_gcd) = (0, 0);

    for (i, &num) in nums.iter().enumerate() {
        let gcd = match i {
            0 => *gcd_accum_right[..gcd_accum_len].iter().nth_back(1).unwrap(),
            i if i == n - 1 => *gcd_accum_left[..gcd_accum_len].iter().nth_back(1).unwrap(),
            _ => get_gcd(gcd_accum_left[i], gcd_accum_right[n - 1 - i]),
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
