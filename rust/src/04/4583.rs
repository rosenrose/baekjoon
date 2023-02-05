use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    'outer: for input in buf.lines().take_while(|&input| input != "#") {
        let mut mirror = String::new();

        for ch in input.chars().rev() {
            match ch {
                'b' => mirror.push('d'),
                'd' => mirror.push('b'),
                'p' => mirror.push('q'),
                'q' => mirror.push('p'),
                'i' | 'o' | 'v' | 'w' | 'x' => mirror.push(ch),
                _ => {
                    println!("INVALID");
                    continue 'outer;
                }
            }
        }

        println!("{mirror}");
    }
}
