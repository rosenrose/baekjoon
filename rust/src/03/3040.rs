fn main() {
    let mut buf = String::new();

    let heights = parse_int_vec_lines(&mut buf, 9);

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

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}

fn parse_int_vec_lines(buf: &mut String, n: i32) -> Vec<i32> {
    (0..n)
        .map(|_| {
            read_line(buf);
            parse_int(buf)
        })
        .collect()
}