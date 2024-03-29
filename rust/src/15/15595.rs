use std::collections::HashMap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    const ADMIN: &str = "megalusion";

    let n: usize = input.next().unwrap().parse().unwrap();
    let mut submit_infos = HashMap::with_capacity(n >> 1);

    for _ in 0..n {
        let [_, id, result, ..] = [(); 7].map(|_| input.next().unwrap());

        if id == ADMIN {
            continue;
        }

        let is_accepted = result == "4";

        submit_infos
            .entry(id)
            .and_modify(|(count, is_ac)| {
                if *is_ac {
                    return;
                }

                *count += 1;
                *is_ac = is_accepted;
            })
            .or_insert((1, is_accepted));
    }
    // println!("{submit_infos:?}");
    let mut submit_count = 0;
    let accepted_user_count = submit_infos
        .iter()
        .filter(|(_, &(count, is_ac))| {
            if is_ac {
                submit_count += count;
            }

            is_ac
        })
        .count();

    if accepted_user_count == 0 {
        println!("0");
        return;
    }

    let ratio = (accepted_user_count as f64 / submit_count as f64) * 100.0;

    println!("{ratio:.9}");
}
