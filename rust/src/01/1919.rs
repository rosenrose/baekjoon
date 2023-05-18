use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().map(|s: &str| {
        let mut count = [0; 26];

        for ch in s.as_bytes() {
            count[(ch - b'a') as usize] += 1;
        }

        count
    });

    let a_counts = input.next().unwrap();
    let b_counts = input.next().unwrap();
    // println!("{a_counts:?} {b_counts:?}");
    let get_delete_count = |a: &[i32], b: &[i32]| {
        a.iter()
            .zip(b)
            .map(|(a_count, b_count)| (a_count - b_count).max(0))
            .sum::<i32>()
    };

    let a_delete_count = get_delete_count(&a_counts, &b_counts);
    let b_delete_count = get_delete_count(&b_counts, &a_counts);

    println!("{}", a_delete_count + b_delete_count);
}
