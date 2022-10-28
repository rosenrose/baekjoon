fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();

    for _ in 0..n {
        read_line(&mut buf);

        let group_of_o = buf.trim().split(|c| c == 'X').filter(|c| !c.is_empty());

        let scores = group_of_o.map(|o_str| o_str.chars().enumerate().map(|(i, _)| i + 1));

        let total = scores.flatten().sum::<usize>();

        println!("{total}");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

/*
fn group_by_o(str: &String) -> Vec<Vec<char>> {
    let mut group_of_o = Vec::new();
    let mut arr = Vec::new();

    for char in str.chars() {
        match char {
            'O' => arr.push(char),
            'X' => {
                if !arr.is_empty() {
                    group_of_o.push(arr.clone());
                    arr.clear();
                }
            },
            _ => (),
        }
    }

    if !arr.is_empty() {
        group_of_o.push(arr);
    }

    group_of_o
}
*/
