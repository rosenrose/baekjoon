#[derive(Clone)]
struct Matrix(Vec<Vec<i64>>);

impl Matrix {
    fn mul_rem(&self, other: &Self, m: i64) -> Self {
        Matrix(
            self.0
                .iter()
                .map(|row| {
                    (0..other.0[0].len())
                        .map(|i| {
                            row.iter()
                                .enumerate()
                                .fold(0, |acc, (j, num)| (acc + (num * other.0[j][i])) % m)
                        })
                        .collect()
                })
                .collect(),
        )
    }

    fn pow_rem(&self, exp: i64, m: i64) -> Self {
        if exp == 1 {
            return self.clone();
        }

        let mut half = self.pow_rem(exp >> 1, m);
        half = half.mul_rem(&half, m);

        if exp & 1 == 0 {
            half
        } else {
            half.mul_rem(self, m)
        }
    }
}

const M: i64 = 1_000_000_007;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [k, n] = parse_int_vec(&buf)[..] else { return };

    println!("{}", s(k, n));
}

fn s(k: i64, n: i64) -> i64 {
    if k == 0 {
        return n;
    }
    if n == 1 {
        return 1;
    }

    let matrix = Matrix(
        (0..=k + 1)
            .map(|i| (0..=k + 1).map(|j| i64::from(j <= i)).collect())
            .collect(),
    );

    let result = matrix.pow_rem(n - 1, M);

    result.0[k as usize + 1]
        .iter()
        .fold(0, |acc, num| (acc + num) % M)
}

fn parse_int_vec(buf: &str) -> Vec<i64> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
