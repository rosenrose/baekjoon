use std::cmp::Ordering;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [br, bc, dr, dc, jr, jc] = [(); 6].map(|_| input.next().unwrap());
    let bessie_dist = br.abs_diff(jr).max(bc.abs_diff(jc));
    let daisy_dist = dr.abs_diff(jr) + dc.abs_diff(jc);

    match bessie_dist.cmp(&daisy_dist) {
        Ordering::Less => println!("bessie"),
        Ordering::Equal => println!("tie"),
        Ordering::Greater => println!("daisy"),
    }
}
