fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: usize = buf.trim().parse().unwrap();
    let room = parse_str_vec_lines(&mut buf, n);

    let count_rest = |row: &String| {
        row.split('X')
            .filter(|s| !s.is_empty() && s.contains(".."))
            .count()
    };

    let horizontal_count = room.iter().map(count_rest).sum::<usize>();

    let room_inversed = room.iter().fold(vec![String::new(); n], |mut acc, row| {
        for (i, a) in acc.iter_mut().enumerate() {
            a.push(row.chars().nth(i).unwrap());
        }

        acc
    });
    // println!("{room_inversed:?}");
    let vertical_count = room_inversed.iter().map(count_rest).sum::<usize>();

    println!("{horizontal_count} {vertical_count}");
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_str_vec_lines(buf: &mut String, n: usize) -> Vec<String> {
    (0..n)
        .map(|_| {
            read_line(buf);
            buf.trim().to_string()
        })
        .collect()
}
