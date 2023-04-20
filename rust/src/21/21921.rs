use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let (_, x) = (input.next(), input.next().unwrap() as usize);
    let visits: Vec<_> = input.collect();

    let (mut visit_count, mut max_visit) = (0, 0);
    let mut term_count = 0;

    for (i, window) in visits.windows(x).enumerate() {
        if i == 0 {
            visit_count = window.iter().sum();
        } else {
            visit_count += window[x - 1];
        }

        if visit_count == max_visit {
            term_count += 1;
        }

        if visit_count > max_visit {
            max_visit = visit_count;
            term_count = 1;
        }

        visit_count -= window[0];
    }

    if max_visit == 0 {
        println!("SAD");
        return;
    }

    println!("{max_visit}\n{term_count}");
}
