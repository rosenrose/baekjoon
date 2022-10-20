use std::cmp::Ordering;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let arr = parse_int_vec(&buf);
    let (mut ascend, mut descend) = (0, 0);

    for i in 0..arr.len() - 1 {
        match arr[i].cmp(&arr[i + 1]) {
            Ordering::Less => ascend += 1,
            Ordering::Greater => descend += 1,
            _ => (),
        };
    }

    if ascend == 0 {
        println!("descending");
        return;
    }
    if descend == 0 {
        println!("ascending");
        return;
    }

    println!("mixed");
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
