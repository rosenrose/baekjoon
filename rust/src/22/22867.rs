use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    let mut times: Vec<_> = (0..parse_int(input()))
        .flat_map(|_| {
            let (enter, leave) = (parse_time(input()), parse_time(input()));
            [(enter, true), (leave, false)]
        })
        .collect();
    times.sort_unstable();
    // println!("{times:?}");
    let (mut count, mut max_count) = (0, 1);

    for (_, is_enter) in times {
        if is_enter {
            count += 1;
        } else {
            count -= 1;
        }

        max_count = count.max(max_count);
    }

    println!("{max_count}");
}

fn parse_time(time: &str) -> i32 {
    let mut it = time.split([':', '.']).map(parse_int);
    let [hour, minute, second, millisecond] = [(); 4].map(|_| it.next().unwrap());
    let seconds = hour * (60 * 60) + minute * 60 + second;

    seconds * 1000 + millisecond
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
