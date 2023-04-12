use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let mut consume = 2;
    let mut consume_accum = 0;
    let mut prev_phone = 0;

    for cur_phone in input.skip(1) {
        if cur_phone == prev_phone {
            consume *= 2;
        } else {
            consume = 2;
        }

        if consume_accum + consume >= 100 {
            (consume, consume_accum, prev_phone) = (2, 0, 0);
            continue;
        }

        consume_accum += consume;
        prev_phone = cur_phone;
    }

    println!("{consume_accum}");
}
