use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let n: i32 = input.next().unwrap().parse().unwrap();

    for _ in 0..n {
        let sounds = input.next().unwrap();
        let mut animals = Vec::new();

        loop {
            let line = input.next().unwrap();

            if line == "what does the fox say?" {
                break;
            }

            animals.push(line.split(' ').next_back().unwrap());
        }

        let fox: Vec<_> = sounds.split(' ').filter(|s| !animals.contains(s)).collect();

        println!("{}", fox.join(" "));
    }
}
