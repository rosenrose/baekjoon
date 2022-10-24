fn main() {
    let mut buf = String::new();

    let heights: Vec<i32> = (0..9)
        .map(|_| {
            read_line(&mut buf);
            parse_int(&buf)
        })
        .collect();

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

    let mut littles: Vec<&i32> = heights
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
