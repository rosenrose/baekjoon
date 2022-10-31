fn main() {
    let mut buf = String::new();

    let angles = parse_int_vec_lines(&mut buf, 3);

    if angles.iter().sum::<i32>() != 180 {
        println!("Error");
        return;
    }

    if angles.iter().all(|&angle| angle == 60) {
        println!("Equilateral");
        return;
    }

    if angles[0] == angles[1] || angles[0] == angles[2] || angles[1] == angles[2] {
        println!("Isosceles");
        return;
    }

    println!("Scalene");
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
