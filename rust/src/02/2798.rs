use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [_, m] = [(); 2].map(|_| input.next().unwrap());
    let arr: Vec<_> = input.collect();

    let len = arr.len();
    let mut min_sum = 0;

    for i in 0..len - 2 {
        for j in i + 1..len - 1 {
            for k in j + 1..len {
                let sum = arr[i] + arr[j] + arr[k];

                if sum > m {
                    continue;
                }

                if m - sum < m - min_sum {
                    min_sum = sum;
                }
            }
        }
    }

    println!("{min_sum}");
}
