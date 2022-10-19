fn main() {
    let mut buf = String::new();

    const N: usize = 3;
    let mut x_points = [0; N];
    let mut y_points = [0; N];

    for i in 0..N {
        read_line(&mut buf);
        let mut nums = buf.split_whitespace().map(|s| s.parse::<i32>().unwrap());

        x_points[i] = nums.next().unwrap();
        y_points[i] = nums.next().unwrap();
    }

    for x in x_points {
        if x_points.iter().filter(|&n| *n == x).count() == 1 {
            print!("{x} ");
            break;
        }
    }
    for y in y_points {
        if y_points.iter().filter(|&n| *n == y).count() == 1 {
            println!("{y}");
            return;
        }
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
