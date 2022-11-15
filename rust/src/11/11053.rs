fn main() {
    let mut buf = String::new();
    read_line(&mut buf);
    read_line(&mut buf);

    let mut a = buf.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    let mut lis = vec![a.next().unwrap()];

    for num in a {
        if num > *lis.last().unwrap() {
            lis.push(num);
        }

        let pos = match lis.binary_search(&num) {
            Ok(i) => i,
            Err(i) => i,
        };

        lis[pos] = num;
    }

    println!("{}", lis.len());
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
