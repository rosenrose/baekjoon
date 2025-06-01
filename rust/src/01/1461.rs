use std::io;

const MAX: usize = 50;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [n, m] = [(); 2].map(|_| input.next().unwrap() as usize);
    let mut coords = [0; MAX];

    for (i, num) in input.enumerate() {
        coords[i] = num;
    }

    coords[..n].sort();

    let (left, right) = coords[..n].split_at(coords[..n].partition_point(|&x| x < 0));
    // println!("{left:?} {right:?}");
    let first = left.first().unwrap_or(&0).abs();
    let last = right.last().unwrap_or(&0).abs();

    let steps = if first > last {
        let left_steps: i32 = left[m.min(left.len())..]
            .chunks(m)
            .map(|chunk| chunk[0].abs() * 2)
            .sum();
        let right_steps: i32 = right
            .rchunks(m)
            .map(|chunk| chunk[chunk.len() - 1] * 2)
            .sum();

        first + left_steps + right_steps
    } else {
        let left_steps: i32 = left.chunks(m).map(|chunk| chunk[0].abs() * 2).sum();
        let right_steps: i32 = right[..right.len().saturating_sub(m)]
            .rchunks(m)
            .map(|chunk| chunk[chunk.len() - 1] * 2)
            .sum();

        left_steps + right_steps + last
    };

    println!("{steps}");
}
