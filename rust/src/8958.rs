fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();

    for _ in 0..n {
        read_line(&mut buf);

        let group_of_o = group_by_o(&buf);
        // println!("{group_of_o:?}");
        let scores = group_of_o
            .iter()
            .map(|arr| arr.iter().enumerate().map(|(i, _)| i as i32 + 1));

        let total = scores.fold(0, |a, b| a + b.sum::<i32>());

        println!("{total}");
    }
}

fn group_by_o(input: &String) -> Vec<Vec<char>> {
    let mut group_of_o = Vec::new();
    let mut arr = Vec::new();

    for ox in input.chars() {
        match ox {
            'O' => arr.push(ox),
            'X' => {
                if arr.len() > 0 {
                    group_of_o.push(arr.clone());
                    arr.clear();
                }
            }
            _ => (),
        }
    }

    if arr.len() > 0 {
        group_of_o.push(arr);
    }

    group_of_o
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
