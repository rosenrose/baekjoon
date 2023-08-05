use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [n, _] = [(); 2].map(|_| input.next().unwrap());
    let mut queue: VecDeque<_> = (1..=n).collect();
    let mut count = 0;

    for i in input {
        let pos = queue.iter().position(|&num| num == i).unwrap();

        while let Some(&num) = queue.front() {
            if num == i {
                queue.pop_front();
                break;
            }

            if pos <= queue.len() / 2 {
                queue.rotate_left(1);
            } else {
                queue.rotate_right(1);
            }

            count += 1;
        }
    }

    println!("{count}");
}
