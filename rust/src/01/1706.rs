use std::io;

const MAX: usize = 20;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let [height, width] = [(); 2].map(|_| input.next().unwrap().parse::<usize>().unwrap());
    let mut room = [(); MAX].map(|_| String::new());
    let mut room_inversed = [(); MAX].map(|_| String::new());

    for (r, row) in input.map(str::to_owned).enumerate() {
        room[r] = row;
    }

    for r in 0..height {
        room_inversed[r] = (0..width).flat_map(|c| room[c].chars().nth(r)).collect();
    }

    let horizontal = room[..height].iter().flat_map(get_words);
    let vertical = room_inversed[..height].iter().flat_map(get_words);

    println!("{}", horizontal.chain(vertical).min().unwrap());
}

fn get_words(row: &String) -> impl Iterator<Item = &str> {
    row.split('#').filter(|s| s.len() >= 2)
}
