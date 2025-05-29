use std::collections::HashMap;
use std::fmt::Write;
use std::io;

const GROUP_MAX: usize = 10 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();
    let mut output = String::new();

    let (u, f) = input.next().unwrap().split_once(' ').unwrap();
    let (u, f) = (parse_int(u), parse_int(f));
    let mut group_infos = HashMap::with_capacity(u);

    for _ in 0..u {
        let user_info = input.next().unwrap();
        let mut groups = [""; GROUP_MAX];
        let mut groups_len;

        if let Some((user, group_info)) = user_info.split_once(' ') {
            groups[0] = user;
            groups_len = 1;

            for group in group_info.split(',') {
                groups[groups_len] = group;
                groups_len += 1;
            }
        } else {
            groups[0] = user_info;
            groups_len = 1;
        }

        let user = groups[0];

        for &group in &groups[..groups_len] {
            group_infos
                .entry(group)
                .and_modify(|users: &mut Vec<_>| (*users).push(user))
                .or_insert(vec![user]);
        }
    }

    let mut input = input.flat_map(|s| s.split(' '));
    let mut input = || input.next().unwrap();
    let mut file_infos = HashMap::with_capacity(f);

    for _ in 0..f {
        let [file, perm, owner, group] = [(); 4].map(|_| input());
        let mut perms = [0; 3];

        for (i, ch) in perm.as_bytes().iter().enumerate() {
            perms[i] = ch - b'0';
        }

        file_infos.insert(file, (perms, owner, group));
    }
    // println!("{group_infos:?}\n{file_infos:?}");
    for _ in 0..parse_int(input()) {
        let [user, file, op] = [(); 3].map(|_| input());
        let op = match op {
            "R" => 4,
            "W" => 2,
            "X" => 1,
            _ => unreachable!(),
        };

        let (perms, owner, group) = &file_infos[file];
        let permission = if owner == &user {
            perms[0]
        } else if group_infos[group].contains(&user) {
            perms[1]
        } else {
            perms[2]
        };

        writeln!(output, "{}", (op & permission == op) as u8).unwrap();
    }

    print!("{output}");
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
