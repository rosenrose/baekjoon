use std::collections::HashSet;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let unique_users: HashSet<_> = buf
        .lines()
        .skip(1)
        .map(|input| {
            let (localpart, provider) = input.split_once('@').unwrap();
            let username = if let Some((name, _)) = localpart.split_once('+') {
                name
            } else {
                localpart
            };

            (username.replace('.', ""), provider)
        })
        .collect();

    println!("{}", unique_users.len());
}
