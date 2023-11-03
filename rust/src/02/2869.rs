fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [a, b, v] = parse_int_vec(&buf)[..] else {
        return;
    };
    let climb_per_day = a - b;
    let least_height_before_arrive = v - a;

    let step_before_arrive = if least_height_before_arrive == 0 {
        0
    } else {
        least_height_before_arrive.div_ceil(climb_per_day)
    };

    println!("{}", step_before_arrive + 1);
}

fn parse_int_vec(buf: &str) -> Vec<u32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
