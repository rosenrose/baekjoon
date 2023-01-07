use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());

    for _ in 0..input.next().unwrap() {
        let (k, n) = (input.next().unwrap(), input.next().unwrap());
        let residents = residents_at_floor(k, n);

        println!("{}", residents[n - 1])
    }
}

fn residents_at_floor(floor: usize, room_num: usize) -> Vec<usize> {
    let mut residents: Vec<_> = (1..=room_num).collect();

    for _ in 0..floor {
        for i in 1..residents.len() {
            residents[i] += residents[i - 1];
        }
    }

    residents
}
