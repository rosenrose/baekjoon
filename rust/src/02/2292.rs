fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();

    let hive_num = |num| if num == 1 { 1 } else { (num - 1) * 6 };

    let mut step = 1;
    let mut hive_num_accum = hive_num(1);

    while hive_num_accum < n {
        step += 1;
        hive_num_accum += hive_num(step);
    }

    println!("{step}");
}
