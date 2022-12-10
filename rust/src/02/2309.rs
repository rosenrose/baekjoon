use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let heights: Vec<_> = buf.lines().map(|s| s.parse::<i32>().unwrap()).collect();
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

    let mut littles: Vec<_> = heights
        .iter()
        .filter(|h| !non_littles.contains(h))
        .collect();

    littles.sort();

    for little in littles {
        println!("{little}");
    }
}
