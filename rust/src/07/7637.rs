use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    'outer: while let Ok(n @ 1..) = input.next().unwrap().parse() {
        let mut times: Vec<_> = input
            .by_ref()
            .take(n)
            .flat_map(|time| {
                let (start, end) = time.split_once('-').unwrap();
                [(start, true), (end, false)]
            })
            .collect();
        times.sort();

        let mut count = 0;

        for (_, is_start) in times {
            if is_start {
                count += 1;
            } else {
                count -= 1;
            }

            if count > 1 {
                println!("conflict");
                continue 'outer;
            }
        }

        println!("no conflict");
    }
}
