use std::collections::VecDeque;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let (_, size) = (input.next(), input.next().unwrap());
    let mut monotone_queue = VecDeque::with_capacity(size as usize);

    for (i, num) in input.enumerate() {
        while let Some(&(_, front_idx)) = monotone_queue.front() {
            if front_idx >= (i as i32 + 1) - size {
                break;
            }

            monotone_queue.pop_front();
        }

        while let Some(&(back, _)) = monotone_queue.back() {
            if back <= num {
                break;
            }

            monotone_queue.pop_back();
        }

        monotone_queue.push_back((num, i as i32));
        // println!("{monotone_queue:?}");
        write!(output, "{} ", monotone_queue.front().unwrap().0).unwrap();
    }

    print!("{output}");
}
