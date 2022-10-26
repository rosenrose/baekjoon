use std::fmt;

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
    read_line(&mut buf);

    let cake_a = parse_cake(&buf);
    read_line(&mut buf);

    let cake_c = parse_cake(&buf);

    let cake_b = cake_c.cake_op_inverse(cake_a);

    println!("{cake_b}");
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_cake(buf: &String) -> Cake {
    let mut nums = buf.split_whitespace().map(|s| s.parse().unwrap());

    Cake {
        x: nums.next().unwrap(),
        y: nums.next().unwrap(),
        z: nums.next().unwrap(),
    }
}
