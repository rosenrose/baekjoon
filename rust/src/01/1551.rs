use std::io;

const MAX: usize = 20;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let [n, k] = [(); 2].map(|_| parse_int(input.next().unwrap()) as usize);
    let mut arr = [0; MAX];

    for (i, num) in input.next().unwrap().split(',').map(parse_int).enumerate() {
        arr[i] = num;
    }

    for cnt in 0..k {
        for idx in 0..n - 1 - cnt {
            arr[idx] = arr[idx + 1] - arr[idx];
        }
    }

    println!(
        "{}",
        format!("{:?}", &arr[..n - k]).replace(['[', ']', ' '], "")
    );
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
