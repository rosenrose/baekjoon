use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<i32>);

    let mut heights = [0; 9];

    for (i, height) in input.enumerate() {
        heights[i] = height;
    }

    let sum: i32 = heights.iter().sum();

    let mut non_littles = [0; 2];

    'outer: for i in 0..heights.len() - 1 {
        for j in i + 1..heights.len() {
            if heights[i] + heights[j] == sum - 100 {
                non_littles[0] = heights[i];
                non_littles[1] = heights[j];
                break 'outer;
            }
        }
    }

    let mut littles = [0; 7];

    for (i, &height) in heights
        .iter()
        .filter(|h| !non_littles.contains(h))
        .enumerate()
    {
        littles[i] = height;
    }

    littles.sort();

    for little in littles {
        println!("{little}");
    }
}
