fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [a, b, v] = parse_int_vec(&buf)[..] {
        let climb_per_day = a - b;
        let least_height_before_arrive = v - a;

        let step_before_arrive = if least_height_before_arrive == 0 {
            0
        } else {
            (least_height_before_arrive as f64 / climb_per_day as f64).ceil() as i32
        };

        println!("{}", step_before_arrive + 1);
    }
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
