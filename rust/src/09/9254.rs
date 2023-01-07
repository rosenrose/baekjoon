use std::fmt;
use std::io;

struct Matrix {
    matrix: Vec<Vec<f32>>,
}

impl Matrix {
    fn from(matrix: Vec<Vec<f32>>) -> Self {
        Self { matrix }
    }

    fn inverse(&mut self) -> Option<Self> {
        // if is_zero(self.determinant()) {
        //     return None;
        // }
        // Some(self.adjoint().div(self.determinant()))

        let size = self.matrix.len();
        let mut inv = Self::from(
            (0..size)
                .map(|i| (0..size).map(|j| if i == j { 1.0 } else { 0.0 }).collect())
                .collect(),
        );

        for j in 0..size {
            if is_zero(self.matrix[j][j]) {
                match (j + 1..size).find(|&k| !is_zero(self.matrix[k][j])) {
                    Some(k) => {
                        self.row_operation(k, j, j, 1.0);
                        inv.row_operation(k, j, 0, 1.0);
                    }
                    None => return None,
                }
            }

            let diagonal = self.matrix[j][j];

            for k in 0..size {
                if k >= j {
                    self.matrix[j][k] /= diagonal;
                }
                inv.matrix[j][k] /= diagonal;
            }

            for i in 0..size {
                if i == j {
                    continue;
                }

                let mul = -self.matrix[i][j];

                self.row_operation(j, i, j, mul);
                inv.row_operation(j, i, 0, mul);
            }
        }

        Some(inv)
    }

    fn row_operation(&mut self, src: usize, dst: usize, start_col: usize, mul: f32) {
        for i in start_col..self.matrix[dst].len() {
            self.matrix[dst][i] += self.matrix[src][i] * mul;
        }
    }

    // fn div(&self, div: f32) -> Self {
    //     Self::from(
    //         self.matrix
    //             .iter()
    //             .map(|row| row.iter().map(|cell| cell / div).collect())
    //             .collect(),
    //     )
    // }

    // fn adjoint(&self) -> Self {
    //     self.cofactor().transpose()
    // }

    // fn cofactor(&self) -> Self {
    //     Self::from(
    //         self.matrix
    //             .iter()
    //             .enumerate()
    //             .map(|(i, row)| {
    //                 row.iter()
    //                     .enumerate()
    //                     .map(|(j, _)| {
    //                         (if (i + j) % 2 == 0 { 1.0 } else { -1.0 })
    //                             * self.minor(i, j).determinant()
    //                     })
    //                     .collect()
    //             })
    //             .collect(),
    //     )
    // }

    // fn minor(&self, r: usize, c: usize) -> Self {
    //     Self::from(
    //         self.matrix
    //             .iter()
    //             .enumerate()
    //             .filter_map(|(i, row)| {
    //                 (i != r).then(|| {
    //                     row.iter()
    //                         .enumerate()
    //                         .filter_map(|(j, &cell)| (j != c).then(|| cell))
    //                         .collect()
    //                 })
    //             })
    //             .collect(),
    //     )
    // }

    // fn transpose(&self) -> Self {
    //     Self::from(
    //         (0..self.matrix[0].len())
    //             .map(|i| self.matrix.iter().map(|row| row[i]).collect())
    //             .collect(),
    //     )
    // }

    // fn determinant(&self) -> f32 {
    //     if self.matrix.len() == 1 {
    //         return self.matrix[0][0];
    //     }
    //     if self.matrix.len() == 2 {
    //         return self.matrix[0][0] * self.matrix[1][1] - self.matrix[0][1] * self.matrix[1][0];
    //     }

    //     let i = 0;

    //     self.matrix[i]
    //         .iter()
    //         .enumerate()
    //         .map(|(j, cell)| {
    //             (if (i + j) % 2 == 0 { 1.0 } else { -1.0 }) * cell * self.minor(i, j).determinant()
    //         })
    //         .sum()
    // }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.matrix.iter() {
            for cell in row {
                write!(f, "{cell:.6} ").unwrap();
            }
            writeln!(f, "").unwrap();
        }

        write!(f, "")
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<f32>().unwrap());

    let n = input.next().unwrap() as i32;
    let mut a = Matrix::from(
        (0..n)
            .map(|_| (0..n).map(|_| input.next().unwrap()).collect())
            .collect(),
    );

    match a.inverse() {
        Some(i) => print!("{i}"),
        None => println!("no inverse"),
    }
}

fn is_zero(num: f32) -> bool {
    (num - 0.0).abs() < 1e-10
}
