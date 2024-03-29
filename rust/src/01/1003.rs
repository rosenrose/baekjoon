use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<i32>);

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
