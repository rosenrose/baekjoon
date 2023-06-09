use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let mut cows = [None; 11];
    let mut count = 0;

    for (num, dir) in (0..input()).map(|_| (input(), input())) {
        if let Some(prev_dir) = cows[num] {
            if prev_dir != dir {
                count += 1;
            }
        };

        cows[num] = Some(dir);
    }

    println!("{count}");
}
