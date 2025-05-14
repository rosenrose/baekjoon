fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n, k, m] = parse_int_vec(&buf)[..] else {
        return;
    };
    let mut combination_rem = vec![vec![0; m]; m];

    for i in 0..m {
        combination_rem[i][0] = 1;
        combination_rem[i][i] = 1;

        for j in 1..i {
            combination_rem[i][j] = (combination_rem[i - 1][j] + combination_rem[i - 1][j - 1]) % m;
        }
    }

    let n_digits = get_digits(n, m);
    let k_digits = get_digits(k, m);

    let mut rem = 1;

    for (i, &n_digit) in n_digits.iter().enumerate() {
        let k_digit = *k_digits.get(i).unwrap_or(&0);

        if n_digit < k_digit {
            println!("0");
            return;
        }

        rem = (rem * combination_rem[n_digit][k_digit]) % m;
    }

    println!("{rem}");
}

fn get_digits(mut num: usize, radix: usize) -> Vec<usize> {
    let mut digits = Vec::new();

    while num > 0 {
        digits.push(num % radix);
        num /= radix;
    }

    digits
}

fn parse_int_vec(buf: &str) -> Vec<usize> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
