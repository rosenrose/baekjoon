use std::fmt;
use std::io;
use std::ops::Mul;

#[derive(Clone)]
struct Matrix(Vec<Vec<i32>>);

impl Matrix {
    fn rem(&self, m: i32) -> Self {
        Self(
            self.0
                .iter()
                .map(|row| row.iter().map(|num| num % m).collect())
                .collect(),
        )
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(
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
        for row in self.0.iter() {
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
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);
    let mut input = || input.next().unwrap();

    let (n, b) = (input(), input());
    let a = Matrix(
        (0..n)
            .map(|_| (0..n).map(|_| input() as i32).collect())
            .collect(),
    );

    print!("{}", pow_rem(&a, b));
}

fn pow_rem(base: &Matrix, exp: i64) -> Matrix {
    if exp == 1 {
        return base.rem(M);
    }

    let mut rem = pow_rem(base, exp / 2);
    rem = (rem.clone() * rem).rem(M);

    if exp % 2 == 0 {
        rem
    } else {
        (rem * base.rem(M)).rem(M)
    }
}
