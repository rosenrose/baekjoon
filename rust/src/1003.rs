fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = parse_int(&buf);

    for _ in 0..n {
        read_line(&mut buf);
        let num = parse_int(&buf);

        let mut count_0 = 1;
        let mut count_1 = 1;

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

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
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
