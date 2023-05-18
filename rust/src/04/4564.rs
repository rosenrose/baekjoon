use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<i32>);

    for mut s in input.take_while(|&input| input != 0) {
        while s >= 10 {
            print!("{s} ");

            s = s
                .to_string()
                .as_bytes()
                .iter()
                .map(|ch| (ch - b'0') as i32)
                .product();
        }

        println!("{s}");
    }
}
