use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let mut cables: Vec<_> = (0..input()).map(|_| (input(), input())).collect();
    cables.sort();
    // println!("{cables:?}");
    let mut lis_temp = vec![cables[0].1];

    for &(_, num) in cables.iter().skip(1) {
        if num > *lis_temp.last().unwrap() {
            lis_temp.push(num);
            continue;
        }

        let i = lis_temp.partition_point(|&n| n < num);
        lis_temp[i] = num;
    }

    println!("{}", cables.len() - lis_temp.len());
}
