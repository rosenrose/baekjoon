use std::fmt;
use std::io;
use std::ops::{Add, Mul};

struct Matrix(Vec<Vec<i64>>);

impl Matrix {
    fn rem(&self, m: i64) -> Self {
        Self(
            self.0
                .iter()
                .map(|row| row.iter().map(|num| num % m).collect())
                .collect(),
        )
    }

    fn pow_rem(&self, exp: i64, m: i64) -> Self {
        if exp == 1 {
            return self.rem(m);
        }

        let mut half = self.pow_rem(exp >> 1, m);
        half = (&half * &half).rem(m);

        if exp & 1 == 0 {
            half
        } else {
            (&half * &self.rem(m)).rem(m)
        }
    }

    fn unit(&self) -> Self {
        let len = self.0.len();

        Self(
            (0..len)
                .map(|i| (0..len).map(|j| i64::from(i == j)).collect())
                .collect(),
        )
    }
}

impl Add for &Matrix {
    type Output = Matrix;

    fn add(self, other: Self) -> Self::Output {
        Matrix(
            self.0
                .iter()
                .zip(&other.0)
                .map(|(row1, row2)| {
                    row1.iter()
                        .zip(row2)
                        .map(|(col1, col2)| col1 + col2)
                        .collect()
                })
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

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.0 {
            for cell in row {
                write!(f, "{cell} ").unwrap();
            }
            writeln!(f, "").unwrap();
        }

        Ok(())
    }
}

const M: i64 = 1_000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let (n, b) = (input.next().unwrap() as usize, input.next().unwrap());
    let a = Matrix((0..n).map(|_| input.by_ref().take(n).collect()).collect());

    println!("{}", power_sum_rem(&a, b));
}

fn power_sum_rem(matrix: &Matrix, exp: i64) -> Matrix {
    if exp == 1 {
        return matrix.rem(M);
    }

    if exp & 1 == 0 {
        let half = exp >> 1;
        (&power_sum_rem(matrix, half) * &(&matrix.unit() + &matrix.pow_rem(half, M)).rem(M)).rem(M)
    } else {
        (&power_sum_rem(matrix, exp - 1) + &matrix.pow_rem(exp, M)).rem(M)
    }
}
