use std::ops::Mul;

struct Matrix(Vec<Vec<i64>>);

impl Matrix {
    fn rem(&self, m: i64) -> Self {
        Self(
            self.0
                .iter()
                .map(|row| row.iter().map(|num| (num + m) % m).collect())
                .collect(),
        )
    }
}

impl Mul for &Matrix {
    type Output = Matrix;

    fn mul(self, other: Self) -> Self::Output {
        Matrix(
            self.0
                .iter()
                .map(|row| {
                    (0..other.0[0].len())
                        .map(|i| {
                            row.iter()
                                .enumerate()
                                .map(|(j, num)| num * other.0[j][i])
                                .sum()
                        })
                        .collect()
                })
                .collect(),
        )
    }
}

const M: i64 = 1_000_000_007;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i64 = buf.trim().parse().unwrap();

    if n % 2 == 1 {
        println!("0");
        return;
    }

    println!("{}", matrix_pow_rem(n / 2));
}

fn matrix_pow_rem(n: i64) -> i64 {
    if n == 1 {
        return 3;
    }
    if n == 2 {
        return 11;
    }

    let mut matrix = Matrix(vec![vec![4, -1], vec![1, 0]]);
    matrix = pow_rem(&mut matrix, n - 2);
    // println!("{:?}", matrix.0);
    ((matrix.0[0][0] * 11) % M + (matrix.0[0][1] * 3) % M) % M
}

fn pow_rem(base: &Matrix, exp: i64) -> Matrix {
    if exp == 1 {
        return base.rem(M);
    }

    let mut rem = pow_rem(base, exp / 2);
    rem = (&rem * &rem).rem(M);

    if exp % 2 == 0 {
        rem
    } else {
        (&rem * &base.rem(M)).rem(M)
    }
}
