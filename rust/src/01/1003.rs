use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf.lines().map(|s| s.parse::<i32>().unwrap());

    for num in input.skip(1) {
        let (mut count_0, mut count_1) = (1, 1);

        match num {
            0 => (count_0, count_1) = (1, 0),
            1 => (count_0, count_1) = (0, 1),
            _ => {
                for _ in 2..num {
                    (count_0, count_1) = (count_1, count_0 + count_1);
                }
            }
        };

        println!("{count_0} {count_1}");
    }
}

/*
0 - 1 0
1 - 0 1
2 - 1 1
3 - 1 2
4 - 2 3
5 - 3 5
6 - 5 8
7 - 8 13
8 - 13 21
9 - 21 34
*/
