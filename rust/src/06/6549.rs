use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    while let Some(n @ 1..) = input.next() {
        let heights: Vec<_> = input.by_ref().take(n as usize).collect();
        let max_area = get_max_area(&heights);

        println!("{max_area}");
    }
}

fn get_max_area(heights: &[i32]) -> usize {
    let len = heights.len();

    if len == 1 {
        return heights[0] as usize;
    }

    let mid = len >> 1;
    let left_area = get_max_area(&heights[..mid]);
    let right_area = get_max_area(&heights[mid..]);

    let (mut left, mut right) = (mid, mid);
    let mut mid_height = heights[mid];
    let mut mid_area = mid_height as usize * (right - left + 1);

    while 0 < left || right < len - 1 {
        // println!("{heights:?} {left} {right}");
        let left_height = if left > 0 { heights[left - 1] } else { -1 };
        let right_height = *heights.get(right + 1).unwrap_or(&-1);

        if left_height > right_height {
            mid_height = mid_height.min(left_height);
            left = left.saturating_sub(1);
        } else {
            mid_height = mid_height.min(right_height);
            right = (right + 1).min(len - 1);
        }

        mid_area = mid_area.max(mid_height as usize * (right - left + 1));
    }

    left_area.max(right_area).max(mid_area)
}
