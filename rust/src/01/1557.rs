fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let k: i32 = buf.trim().parse().unwrap();
    let max = k * 2;
    let mobius_values = get_mobius_values((max as f64).sqrt() as usize);

    println!("{}", binary_search(k, &mobius_values, 0, max));
}

fn get_mobius_values(num: usize) -> Vec<i32> {
    let mut mobius_values = vec![-1; num + 1];
    mobius_values[1] = 1;

    for i in (2..).take_while(|i| i * i <= num) {
        for j in (i * i..=num).step_by(i) {
            if mobius_values[j] == 0 {
                continue;
            }

            mobius_values[j] = if j % (i * i) == 0 {
                0
            } else {
                mobius_values[i] * mobius_values[j / i]
            };
        }
    }

    // for i in 2..=num / 2 {
    //     for j in (i * 2..=num).step_by(i) {
    //         mobius_values[j] -= mobius_values[i];
    //     }
    // }

    mobius_values
}

fn binary_search(value: i32, mobius_values: &Vec<i32>, mut lo: i32, mut hi: i32) -> i32 {
    let mut result = 0;

    while lo <= hi {
        let mid = lo + ((hi - lo) >> 1);

        if get_squre_free_nums_count(mid, mobius_values) < value {
            lo = mid + 1;
        } else {
            result = mid;
            hi = mid - 1;
        }
    }

    result
}

fn get_squre_free_nums_count(num: i32, mobius_values: &Vec<i32>) -> i32 {
    num + (2..)
        .take_while(|i| i * i <= num)
        .map(|i| (num / (i * i)) * mobius_values[i as usize])
        .sum::<i32>()
}
