use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [d_len, _] = [(); 2].map(|_| input.next().unwrap());
    let d: Vec<_> = input.by_ref().take(d_len).collect();
    let m: Vec<_> = input.collect();

    let gcd = get_gcd(m.into_iter());
    let mut lcm = 1;

    for num in d {
        lcm = get_lcm(lcm, num);

        if lcm > gcd {
            println!("0");
            return;
        }
    }

    let mut count = 0;

    for small in (1..).take_while(|i| i * i <= gcd) {
        if gcd % small != 0 {
            continue;
        }

        let big = gcd / small;

        if small % lcm == 0 {
            count += 1;
        }
        if small != big && big % lcm == 0 {
            count += 1;
        }
    }

    println!("{count}");
}

fn get_lcm(a: usize, b: usize) -> usize {
    a / get_gcd([a, b].into_iter()) * b
}

fn get_gcd(nums: impl Iterator<Item = usize>) -> usize {
    nums.reduce(|mut a, mut b| loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    })
    .unwrap()
}
