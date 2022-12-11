use std::fmt;
use std::io::{stdin, Read};

#[derive(Copy, Clone)]
struct Cake {
    x: i32,
    y: i32,
    z: i32,
}

impl Cake {
    fn cake_op(self, other: Self) -> Self {
        Self {
            x: self.z + other.x,
            y: self.y * other.y,
            z: self.x + other.z,
        }
    }

    fn cake_op_inverse(self, other: Self) -> Self {
        Self {
            x: self.x - other.z,
            y: self.y / other.y,
            z: self.z - other.x,
        }
    }
}

impl fmt::Display for Cake {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let cake_a = Cake {
        x: input.next().unwrap(),
        y: input.next().unwrap(),
        z: input.next().unwrap(),
    };
    let cake_c = Cake {
        x: input.next().unwrap(),
        y: input.next().unwrap(),
        z: input.next().unwrap(),
    };

    let cake_b = cake_c.cake_op_inverse(cake_a);

    println!("{cake_b}");
}
