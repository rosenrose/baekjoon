fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let t = parse_int(&buf);

    for _ in 0..t {
        read_line(&mut buf);
        let k = parse_int(&buf);

        read_line(&mut buf);
        let n = parse_int(&buf);

        let residents = residents_at_floor(k, n);

        println!("{}", residents[(n - 1) as usize])
    }
}

fn residents_at_floor(floor: i32, room_num: i32) -> Vec<i32> {
    let mut residents: Vec<i32> = (1..=room_num).collect();

    for _ in 0..floor {
        for i in 0..residents.len() - 1 {
            residents[i + 1] += residents[i];
        }
    }

    residents
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}
