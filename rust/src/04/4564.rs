use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().map(|s| s.parse::<i32>().unwrap());

    for mut s in input.take_while(|&input| input != 0) {
        while s >= 10 {
            print!("{s} ");

            s = s
                .to_string()
                .chars()
                .map(|c| c as i32 - '0' as i32)
                .product();
        }

        println!("{s}");
    }
}
