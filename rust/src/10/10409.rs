use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let (_, t) = (input.next(), input.next().unwrap());
    let (mut time, mut count) = (0, 0);

    for task in input {
        if (t - time) < task {
            break;
        }

        time += task;
        count += 1
    }

    println!("{count}");
}
