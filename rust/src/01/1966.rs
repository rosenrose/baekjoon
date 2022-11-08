use std::collections::VecDeque;

fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let parse_int = |s: &str| s.parse::<usize>().unwrap();
    let n = parse_int(buf.trim());

    'outer: for _ in 0..n {
        read_line(&mut buf);

        let m = buf.split_whitespace().map(parse_int).next_back().unwrap();
        read_line(&mut buf);

        let mut docs: VecDeque<_> = buf
            .split_whitespace()
            .enumerate()
            .map(|(i, s)| (parse_int(s), i))
            .collect();

        if docs.len() == 1 {
            println!("1");
            continue;
        }

        let target = docs[m];
        let mut order = 1;

        while !docs.is_empty() {
            let (mut first_importance, _) = docs[0];

            while docs.iter().skip(1).any(|doc| doc.0 > first_importance) {
                let temp = docs.pop_front().unwrap();
                docs.push_back(temp);

                (first_importance, _) = docs[0];
            }

            if docs.pop_front().unwrap() == target {
                println!("{order}");
                continue 'outer;
            }

            order += 1;
        }
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
