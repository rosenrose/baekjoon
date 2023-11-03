use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<u32>);

    for [h, _, n] in (0..input.next().unwrap()).map(|_| [(); 3].map(|_| input.next().unwrap())) {
        let floor = if n % h == 0 { h } else { n % h };
        let num_from_elevator = n.div_ceil(h);

        println!("{floor}{num_from_elevator:02}");
    }
}
