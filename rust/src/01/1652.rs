use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let room: Vec<_> = buf.lines().skip(1).map(str::to_owned).collect();

    let count_rest = |row: &String| {
        row.split('X')
            .filter(|s| !s.is_empty() && s.contains(".."))
            .count()
    };

    let horizontal_count = room.iter().map(count_rest).sum::<usize>();

    let room_inversed: Vec<String> = (0..room[0].len())
        .map(|i| {
            (0..room.len())
                .map(|j| room[j].chars().nth(i).unwrap())
                .collect()
        })
        .collect();
    // println!("{room_inversed:?}");
    let vertical_count = room_inversed.iter().map(count_rest).sum::<usize>();

    println!("{horizontal_count} {vertical_count}");
}
