fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n = parse_int_vec(&buf)[0];
    let a = parse_matrix(&mut buf, n);
    let b = parse_matrix(&mut buf, n);

    let sum = a
        .iter()
        .zip(b.iter())
        .map(|(row1, row2)| row1.iter().zip(row2.iter()).map(|(col1, col2)| col1 + col2));

    for row in sum {
        for col in row {
            print!("{col} ");
        }
        println!("");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn parse_matrix(buf: &mut String, rows: i32) -> Vec<Vec<i32>> {
    (0..rows)
        .map(|_| {
            read_line(buf);
            parse_int_vec(buf)
        })
        .collect()
}
