use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [mut hour, mut minute, cooking_time] = [(); 3].map(|_| input.next().unwrap());
    minute += cooking_time;

    if minute >= 60 {
        hour = (hour + (minute / 60)) % 24;
        minute %= 60;
    }

    println!("{hour} {minute}");
}
