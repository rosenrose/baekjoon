use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let room: Vec<_> = buf.lines().skip(1).map(str::to_owned).collect();

    let count_rest = |row: &String| {
        row.split('X')
            .filter(|s| !s.is_empty() && s.contains(".."))
            .count()
    };

    let horizontal_count: usize = room.iter().map(count_rest).sum();

    let room_inversed: Vec<String> = (0..room[0].len())
        .map(|i| {
            (0..room.len())
                .flat_map(|j| room[j].chars().nth(i))
                .collect()
        })
        .collect();
    // println!("{room_inversed:?}");
    let vertical_count: usize = room_inversed.iter().map(count_rest).sum();

    println!("{horizontal_count} {vertical_count}");
}
