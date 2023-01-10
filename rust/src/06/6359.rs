use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().map(|s| s.parse::<usize>().unwrap());

    for n in input.skip(1) {
        let mut rooms = vec![false; n];

        for k in 1..=n {
            for i in (k..=n).step_by(k) {
                rooms[i - 1] = !rooms[i - 1];
            }
        }

        println!("{}", rooms.iter().filter(|&&is_open| is_open).count());
    }
}
