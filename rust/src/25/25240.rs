use std::collections::HashMap;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();
    let mut output = String::new();

    let (u, f) = input.next().unwrap().split_once(' ').unwrap();
    let (u, f) = (parse_int(u), parse_int(f));
    let mut group_infos = HashMap::new();

    for _ in 0..u {
        let user_info = input.next().unwrap();
        let (user, groups) = if let Some((user, groups)) = user_info.split_once(' ') {
            let mut g = vec![user];
            g.extend(groups.split(','));

            (user, g)
        } else {
            (user_info, vec![user_info])
        };

        for group in groups {
            group_infos
                .entry(group)
                .and_modify(|g: &mut Vec<_>| (*g).push(user))
                .or_insert(vec![user]);
        }
    }

    let mut input = input.flat_map(|s| s.split(' '));
    let mut input = || input.next().unwrap();
    let mut file_infos = HashMap::new();

    for _ in 0..f {
        let (file, perm, owner, group) = (input(), input(), input(), input());
        let perm: Vec<_> = perm.chars().map(|ch| ch as u8 - '0' as u8).collect();

        file_infos.insert(file, (perm, owner, group));
    }
    // println!("{group_infos:?}\n{file_infos:?}");
    for _ in 0..parse_int(input()) {
        let (user, file, op) = (input(), input(), input());
        let op = match op {
            "R" => 4,
            "W" => 2,
            "X" => 1,
            _ => Default::default(),
        };

        let (perm, owner, group) = &file_infos[file];
        let permission = if owner == &user {
            perm[0]
        } else if group_infos[group].contains(&user) {
            perm[1]
        } else {
            perm[2]
        };

        writeln!(output, "{}", if op & permission == op { 1 } else { 0 }).unwrap();
    }

    print!("{output}");
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
