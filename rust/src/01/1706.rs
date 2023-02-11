use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    let room: Vec<_> = buf.lines().skip(1).map(str::to_owned).collect();
    let room_inversed: Vec<String> = (0..room[0].len())
        .map(|i| {
            (0..room.len())
                .flat_map(|j| room[j].chars().nth(i))
                .collect()
        })
        .collect();

    let get_words = |row: &String| -> Vec<String> {
        row.split('#')
            .filter_map(|s| (s.len() >= 2).then_some(s.to_owned()))
            .collect()
    };

    let horizontal: Vec<_> = room.iter().flat_map(get_words).collect();
    let vertical: Vec<_> = room_inversed.iter().flat_map(get_words).collect();

    let mut words = [horizontal, vertical].concat();
    words.sort();

    println!("{}", words[0]);
}
