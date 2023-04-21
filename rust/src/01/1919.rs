use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let char_count = |s: &str| {
        let mut count = [0; 26];

        for ch in s.chars() {
            count[ch as usize - 'a' as usize] += 1;
        }

        count
    };

    let a_counts = char_count(input.next().unwrap());
    let b_counts = char_count(input.next().unwrap());
    // println!("{a_counts:?} {b_counts:?}");
    let count_diff = |a: [i32; 26], b: [i32; 26]| {
        let mut diff = [0; 26];

        for (i, (a_count, b_count)) in a.iter().zip(b).enumerate() {
            diff[i] = (a_count - b_count).max(0);
        }

        diff
    };

    let a_delete_count: i32 = count_diff(a_counts, b_counts).iter().sum();
    let b_delete_count: i32 = count_diff(b_counts, a_counts).iter().sum();

    println!("{}", a_delete_count + b_delete_count);
}
