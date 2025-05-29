use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let nums = [(); 5].map(|_| input.next().unwrap());
    let mut min_lcm = i32::MAX;

    for i in 0..3 {
        for j in i + 1..4 {
            for k in j + 1..5 {
                let lcm = get_lcm(nums[i], nums[j]);
                let lcm = get_lcm(lcm, nums[k]);

                min_lcm = lcm.min(min_lcm);
            }
        }
    }

    println!("{min_lcm}");
}

fn get_lcm(a: i32, b: i32) -> i32 {
    a / get_gcd(a, b) * b
}

fn get_gcd(mut a: i32, mut b: i32) -> i32 {
    loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    }
}
