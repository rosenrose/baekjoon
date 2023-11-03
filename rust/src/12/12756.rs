use std::cmp::Ordering;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<u32>);

    let [a_atk, a_hp, b_atk, b_hp] = [(); 4].map(|_| input.next().unwrap());
    let a_turns = b_hp.div_ceil(a_atk);
    let b_turns = a_hp.div_ceil(b_atk);

    match a_turns.cmp(&b_turns) {
        Ordering::Less => println!("PLAYER A"),
        Ordering::Equal => println!("DRAW"),
        Ordering::Greater => println!("PLAYER B"),
    }
}
