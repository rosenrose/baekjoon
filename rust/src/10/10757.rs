use std::collections::VecDeque;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut nums = buf.split_whitespace().map(|s| {
        s.chars()
            .map(|c| c as i32 - '0' as i32)
            .collect::<VecDeque<_>>()
    });

    let a = nums.next().unwrap();
    let b = nums.next().unwrap();
    let sum = add_by_array(a, b);

    for i in sum {
        print!("{i}");
    }
}

fn add_by_array(mut a: VecDeque<i32>, mut b: VecDeque<i32>) -> VecDeque<i32> {
    let longer = a.len().max(b.len());

    while a.len() < longer {
        a.push_front(0);
    }
    while b.len() < longer {
        b.push_front(0);
    }

    let mut sum: VecDeque<_> = a.iter().zip(b).map(|(a, b)| a + b).collect();

    for i in (1..longer).rev() {
        if sum[i] < 10 {
            continue;
        }

        sum[i - 1] += sum[i] / 10;
        sum[i] %= 10;
    }

    while sum[0] >= 10 {
        sum.push_front(sum[0] / 10);
        sum[1] %= 10;
    }

    sum
}
