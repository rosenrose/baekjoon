use std::collections::HashSet;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let [start, meeting_end, stream_end] = [(); 3].map(|_| input.next().unwrap());
    let mut members = HashSet::with_capacity(100_000 >> 1);
    let mut count = 0;

    while let [Some(time), Some(name)] = [(); 2].map(|_| input.next()) {
        if time <= start {
            members.insert(name);
            continue;
        }

        if meeting_end <= time && time <= stream_end {
            if members.contains(name) {
                count += 1;
                members.remove(name);
            }
        }
    }

    println!("{count}");
}
