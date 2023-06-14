use std::collections::HashMap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    let [n, m] = [(); 2].map(|_| parse_int(input()));
    let mut gropus = HashMap::with_capacity(n);
    let mut members = HashMap::with_capacity(n);

    for _ in 0..n {
        let group_name = input();
        let count = parse_int(input());

        let mut group: Vec<_> = (0..count)
            .map(|_| {
                let member_name = input();
                members.insert(member_name, group_name);

                member_name
            })
            .collect();
        group.sort();

        gropus.insert(group_name, group);
    }

    for (name, quiz) in (0..m).map(|_| (input(), input())) {
        match quiz {
            "0" => println!("{}", gropus[name].join("\n")),
            "1" => println!("{}", members[name]),
            _ => unreachable!(),
        }
    }
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
