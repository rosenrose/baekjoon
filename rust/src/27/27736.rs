use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let n: i32 = input.next().unwrap().parse().unwrap();
    let (mut pros, mut cons, mut giveup) = (0, 0, 0);

    for vote in input {
        match vote {
            "1" => pros += 1,
            "-1" => cons += 1,
            "0" => giveup += 1,
            _ => (),
        }
    }

    if giveup >= (n + 1) / 2 {
        println!("INVALID");
        return;
    }

    println!("{}", if pros > cons { "APPROVED" } else { "REJECTED" });
}
