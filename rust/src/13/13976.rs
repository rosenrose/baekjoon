use std::ops::Mul;

struct Matrix(Vec<Vec<i64>>);

impl Matrix {
    fn rem(&self, m: i64) -> Self {
        Self(
            self.0
                .iter()
                .map(|row| row.iter().map(|num| num.rem_euclid(m)).collect())
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

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut n: i64 = buf.trim().parse().unwrap();
    const M: i64 = 1_000_000_007;

    if n % 2 == 1 {
        println!("0");
        return;
    }

    n /= 2;

    match n {
        1 => println!("3"),
        2 => println!("11"),
        _ => {
            let mut matrix = Matrix(vec![vec![4, -1], vec![1, 0]]);
            matrix = matrix.pow_rem(n - 2, M);
            // println!("{:?}", matrix.0);
            let count = ((matrix.0[0][0] * 11) % M + (matrix.0[0][1] * 3) % M) % M;

            println!("{count}");
        }
    }
}
