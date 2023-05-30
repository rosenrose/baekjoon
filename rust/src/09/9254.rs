use std::fmt;
use std::io;

struct Matrix(Vec<Vec<f64>>);

impl Matrix {
    fn inverse(&mut self) -> Option<Self> {
        // if is_zero(self.determinant()) {
        //     return None;
        // }
        // Some(self.adjoint().div(self.determinant()))

        let size = self.0.len();
        let mut inv = Self(
            (0..size)
                .map(|i| (0..size).map(|j| if i == j { 1.0 } else { 0.0 }).collect())
                .collect(),
        );

        for j in 0..size {
            if is_zero(self.0[j][j]) {
                let k = (j + 1..size).find(|&k| !is_zero(self.0[k][j]))?;

                self.row_operation(k, j, j, 1.0);
                inv.row_operation(k, j, 0, 1.0);
            }

            let diagonal = self.0[j][j];

            for k in 0..size {
                if k >= j {
                    self.0[j][k] /= diagonal;
                }
                inv.0[j][k] /= diagonal;
            }

            for i in 0..size {
                if i == j {
                    continue;
                }

                let mul = -self.0[i][j];

                self.row_operation(j, i, j, mul);
                inv.row_operation(j, i, 0, mul);
            }
        }

        Some(inv)
    }

    fn row_operation(&mut self, src: usize, dst: usize, start_col: usize, mul: f64) {
        for i in start_col..self.0[dst].len() {
            self.0[dst][i] += self.0[src][i] * mul;
        }
    }

    fn div(&self, div: f64) -> Self {
        Self(
            self.0
                .iter()
                .map(|row| row.iter().map(|cell| cell / div).collect())
                .collect(),
        )
    }

    fn adjoint(&self) -> Self {
        self.cofactor().transpose()
    }

    fn cofactor(&self) -> Self {
        Self(
            self.0
                .iter()
                .enumerate()
                .map(|(i, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(j, _)| {
                            (if (i + j) % 2 == 0 { 1.0 } else { -1.0 })
                                * self.minor(i, j).determinant()
                        })
                        .collect()
                })
                .collect(),
        )
    }

    fn minor(&self, r: usize, c: usize) -> Self {
        Self(
            self.0
                .iter()
                .enumerate()
                .filter_map(|(i, row)| {
                    (i != r).then(|| {
                        row.iter()
                            .enumerate()
                            .filter_map(|(j, &cell)| (j != c).then_some(cell))
                            .collect()
                    })
                })
                .collect(),
        )
    }

    fn transpose(&self) -> Self {
        Self(
            (0..self.0[0].len())
                .map(|i| self.0.iter().map(|row| row[i]).collect())
                .collect(),
        )
    }

    fn determinant(&self) -> f64 {
        if self.0.len() == 1 {
            return self.0[0][0];
        }
        if self.0.len() == 2 {
            return self.0[0][0] * self.0[1][1] - self.0[0][1] * self.0[1][0];
        }

        let i = 0;

        self.0[i]
            .iter()
            .enumerate()
            .map(|(j, cell)| {
                (if (i + j) % 2 == 0 { 1.0 } else { -1.0 }) * cell * self.minor(i, j).determinant()
            })
            .sum()
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.0 {
            for cell in row {
                write!(f, "{cell:.6} ").unwrap();
            }
            writeln!(f, "").unwrap();
        }

        Ok(())
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<f64>);

    let n = input.next().unwrap() as usize;
    let mut a = Matrix((0..n).map(|_| input.by_ref().take(n).collect()).collect());

    if let Some(inv) = a.inverse() {
        print!("{inv}");
    } else {
        println!("no inverse");
    }
}

fn is_zero(num: f64) -> bool {
    (num - 0.0).abs() < 1e-10
}
