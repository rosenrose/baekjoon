struct Sequence {
    num: i32,
    counter: i32,
}

impl Sequence {
    fn new() -> Self {
        Self { num: 1, counter: 1 }
    }
}

impl Iterator for Sequence {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter == 0 {
            self.num += 1;
            self.counter = self.num;
        }

        self.counter -= 1;

        Some(self.num)
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [a, b] = parse_int_vec(&buf)[..] {
        let sequence = Sequence::new();

        println!("{}", sequence.skip(a - 1).take(b - a + 1).sum::<i32>());
    }
}

fn parse_int_vec(buf: &String) -> Vec<usize> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
