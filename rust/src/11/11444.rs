fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i64 = buf.trim().parse().unwrap();

    println!("{}", fibo_rem(n));
}

fn fibo_rem(n: i64) -> i64 {
    if n <= 1 {
        return n;
    }

    let mut fibo_matrix = vec![vec![1, 1], vec![1, 0]];
    fibo_matrix = pow_rem(&mut fibo_matrix, n, 1_000_000_007);

    fibo_matrix[0][1]
}

fn pow_rem(base: &mut Vec<Vec<i64>>, exp: i64, m: i64) -> Vec<Vec<i64>> {
    if exp == 1 {
        return matrix_mod(base, m);
    }

    let mut rem = pow_rem(base, exp / 2, m);
    rem = matrix_mod(
        &matrix_mulitply(&matrix_mod(&rem, m), &matrix_mod(&rem, m)),
        m,
    );

    if exp % 2 == 0 {
        rem
    } else {
        matrix_mod(
            &matrix_mulitply(&matrix_mod(&rem, m), &matrix_mod(&base, m)),
            m,
        )
    }
}

fn matrix_mulitply(a: &Vec<Vec<i64>>, b: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    a.iter()
        .map(|row| {
            (0..a.len())
                .map(|i| row.iter().enumerate().map(|(j, num)| num * b[j][i]).sum())
                .collect()
        })
        .collect()
}

fn matrix_mod(matrix: &Vec<Vec<i64>>, m: i64) -> Vec<Vec<i64>> {
    matrix
        .iter()
        .map(|row| row.iter().map(|num| num % m).collect())
        .collect()
}
