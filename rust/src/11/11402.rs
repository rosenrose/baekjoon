use std::io;

const MAX: usize = 2000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [n, k, m] = [(); 3].map(|_| input.next().unwrap());
    let mut combination_rem = [[0; MAX]; MAX];

    for i in 0..m {
        combination_rem[i][0] = 1;
        combination_rem[i][i] = 1;

        for j in 1..i {
            combination_rem[i][j] = (combination_rem[i - 1][j] + combination_rem[i - 1][j - 1]) % m;
        }
    }
    // 뤼카의 정리
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
