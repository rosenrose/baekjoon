use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<i32>);

    let [start, end, ice, melting, water] = [(); 5].map(|_| input.next().unwrap());
    let mut time = (end - start.max(0)) * water;

    if start < 0 {
        time += -start * ice;
        time += melting;
    }

    println!("{time}");
}
