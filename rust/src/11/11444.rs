use std::ops::Mul;

const M: i64 = 1_000_000_007;

struct Matrix {
    matrix: Vec<Vec<i64>>,
}

impl Matrix {
    fn from(matrix: Vec<Vec<i64>>) -> Self {
        Self { matrix }
    }

    fn rem(self, m: i64) -> Self {
        Self::from(
            self.matrix
                .iter()
                .map(|row| row.iter().map(|num| num % m).collect())
                .collect(),
        )
    }
}

impl Clone for Matrix {
    fn clone(&self) -> Self {
        Self::from((*self.matrix.clone()).to_vec())
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::from(
            self.matrix
                .iter()
                .map(|row| {
                    (0..self.matrix.len())
                        .map(|i| {
                            row.iter()
                                .enumerate()
                                .map(|(j, num)| num * other.matrix[j][i])
                                .sum()
                        })
                        .collect()
                })
                .collect(),
        )
    }
}

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

    let mut fibo_matrix = Matrix::from(vec![vec![1, 1], vec![1, 0]]);
    fibo_matrix = pow_rem(&mut fibo_matrix, n);

    fibo_matrix.matrix[0][1]
}

fn pow_rem(base: &mut Matrix, exp: i64) -> Matrix {
    if exp == 1 {
        return base.clone().rem(M);
    }

    let mut remainder = pow_rem(base, exp / 2);
    remainder = (remainder.clone() * remainder).rem(M);

    if exp % 2 == 0 {
        remainder
    } else {
        (remainder * base.clone().rem(M)).rem(M)
    }
}
