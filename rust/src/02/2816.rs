use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut channels: Vec<_> = buf.lines().skip(1).collect();

    let kbs1_idx = channels.iter().position(|&ch| ch == "KBS1").unwrap();
    print!("{}", "1".repeat(kbs1_idx));

    for i in (1..=kbs1_idx).rev() {
        channels.swap(i, i - 1);
        print!("4");
    }

    let kbs2_idx = channels.iter().position(|&ch| ch == "KBS2").unwrap();
    print!("{}", "1".repeat(kbs2_idx));

    for i in (2..=kbs2_idx).rev() {
        channels.swap(i, i - 1);
        print!("4");
    }
}
