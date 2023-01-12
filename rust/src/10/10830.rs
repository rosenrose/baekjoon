use std::fmt;
use std::io;
use std::ops::Mul;

struct Matrix {
    matrix: Vec<Vec<i32>>,
}

impl Matrix {
    fn from(matrix: Vec<Vec<i32>>) -> Self {
        Self { matrix }
    }

    fn rem(self, m: i32) -> Self {
        Self::from(
            self.matrix
                .iter()
                .map(|row| row.iter().map(|num| num % m).collect())
                .collect(),
        )
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::from(
            self.matrix
                .iter()
                .map(|row| {
                    (0..other.matrix[0].len())
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

impl Clone for Matrix {
    fn clone(&self) -> Self {
        Self::from((*self.matrix.clone()).to_vec())
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.matrix.iter() {
            for cell in row {
                write!(f, "{cell} ").unwrap();
            }
            writeln!(f, "").unwrap();
        }

        write!(f, "")
    }
}

const M: i32 = 1_000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap());
    let mut input = || input.next().unwrap();

    let (n, b) = (input(), input());
    let a = Matrix::from(
        (0..n)
            .map(|_| (0..n).map(|_| input() as i32).collect())
            .collect(),
    );

    print!("{}", pow_rem(&a, b));
}

fn pow_rem(base: &Matrix, exp: i64) -> Matrix {
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
