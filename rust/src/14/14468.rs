fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut paths = [[None; 2]; 26];

    for (i, ch) in buf.trim().as_bytes().iter().enumerate() {
        let idx = (ch - b'A') as usize;

        if paths[idx][0].is_none() {
            paths[idx][0] = Some(i);
        } else {
            paths[idx][1] = Some(i);
        }
    }

    let mut pairs = 0;

    for a in 0..paths.len() - 1 {
        for b in a + 1..paths.len() {
            let (early, late) = (paths[a].min(paths[b]), paths[a].max(paths[b]));

            if late[0] < early[1] && early[1] < late[1] {
                pairs += 1;
            }
        }
    }

    println!("{pairs}");
}
