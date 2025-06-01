use std::io;

const MAX: usize = 1000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut counts = [("", 0); MAX];
    let mut counts_len = 0;
    let mut max_count = 1;

    for title in buf.lines().skip(1) {
        if let Some(count) = counts[..counts_len]
            .iter_mut()
            .find_map(|(t, c)| (*t == title).then_some(c))
        {
            *count += 1;
            max_count = max_count.max(*count);
        } else {
            counts[counts_len] = (title, 1);
            counts_len += 1;
        }
    }

    let best_seller = counts[..counts_len]
        .iter()
        .filter_map(|&(title, count)| (count == max_count).then_some(title))
        .min()
        .unwrap();

    println!("{best_seller}");
}
