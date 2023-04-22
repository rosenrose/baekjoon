use std::collections::VecDeque;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();
    let mut queue = VecDeque::new();
    let mut sum = 0;
    let mut count = 0;

    for num in 1..=n {
        sum += num;
        queue.push_back(num);
        // println!("{queue:?}");
        if sum == n {
            count += 1;
        }

        while sum > n - (num + 1) {
            if let Some(min) = queue.pop_front() {
                sum -= min;
            } else {
                break;
            }
        }
    }

    println!("{count}");
}
