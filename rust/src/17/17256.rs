use std::fmt;
use std::io;

#[derive(Copy, Clone)]
struct Cake(i32, i32, i32);

impl Cake {
    fn cake_op(self, other: Self) -> Self {
        Self(self.2 + other.0, self.1 * other.1, self.0 + other.2)
    }

    fn cake_op_inverse(self, other: Self) -> Self {
        Self(self.0 - other.2, self.1 / other.1, self.2 - other.0)
    }
}

impl fmt::Display for Cake {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let cake_a = Cake(input(), input(), input());
    let cake_c = Cake(input(), input(), input());
    let cake_b = cake_c.cake_op_inverse(cake_a);

    println!("{cake_b}");
}
