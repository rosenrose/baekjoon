use std::fmt;
use std::io;
use std::ops::Mul;

struct Matrix(Vec<Vec<usize>>);

impl Matrix {
    fn rem(&self, m: usize) -> Self {
        Self(
            self.0
                .iter()
                .map(|row| row.iter().map(|num| num % m).collect())
                .collect(),
        )
    }

    fn pow_rem(&self, exp: usize, m: usize) -> Self {
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

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [n, b] = [(); 2].map(|_| input.next().unwrap());
    let a = Matrix((0..n).map(|_| input.by_ref().take(n).collect()).collect());

    print!("{}", a.pow_rem(b, 1000));
}
