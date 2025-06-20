use std::io;

const MAX: usize = 100;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let n: usize = input.next().unwrap().parse().unwrap();
    let mut room = [(); MAX].map(|_| String::new());
    let mut room_inversed = [(); MAX].map(|_| String::new());

    for (r, row) in input.map(str::to_owned).enumerate() {
        room[r] = row;
    }

    for r in 0..n {
        room_inversed[r] = (0..n).flat_map(|c| room[c].chars().nth(r)).collect();
    }
    // println!("{room_inversed:?}");
    let count_rest = |row: &String| row.split('X').filter(|s| s.contains("..")).count();

    let horizontal: usize = room[..n].iter().map(count_rest).sum();
    let vertical: usize = room_inversed[..n].iter().map(count_rest).sum();

    println!("{horizontal} {vertical}");
}
