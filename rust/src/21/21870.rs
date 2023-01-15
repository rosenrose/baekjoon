use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let nums: Vec<_> = input.collect();

    if n == 1 {
        println!("{}", nums[0]);
        return;
    }

    let left_select = gcd_sum(&nums[..n / 2], &nums[n / 2..]);
    let right_select = gcd_sum(&nums[n / 2..], &nums[..n / 2]);

    println!("{}", left_select.max(right_select));
}

fn gcd_sum(select: &[i32], rest: &[i32]) -> i32 {
    let select_gcd = get_gcd(select.iter().copied());
    let n = rest.len();

    select_gcd
        + if n <= 2 {
            rest.iter().sum()
        } else {
            let left_select = gcd_sum(&rest[..n / 2], &rest[n / 2..]);
            let right_select = gcd_sum(&rest[n / 2..], &rest[..n / 2]);

            left_select.max(right_select)
        }
}

fn get_gcd<I>(mut nums: I) -> i32
where
    I: Iterator<Item = i32>,
{
    let mut gcd = nums.next().unwrap();

    for mut num in nums {
        if gcd == 1 {
            return 1;
        }

        gcd = loop {
            if num == 0 {
                break gcd;
            }

            (gcd, num) = (num, gcd % num);
        };
    }

    gcd
}
