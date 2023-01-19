use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let (n, m) = (input(), input());
    let mut required_mileages: Vec<_> = (0..n)
        .map(|_| {
            let (p, max_people) = (input() as usize, input() as usize);
            let mut mileages: Vec<_> = (0..p).map(|_| input()).collect();
            mileages.sort_by(|a, b| b.cmp(a));

            let top = &mileages[..max_people.min(p)];

            if top.len() < max_people {
                1
            } else {
                *top.iter().min().unwrap()
            }
        })
        .collect();
    // println!("{required_mileages:?}");
    required_mileages.sort();

    let count = required_mileages
        .iter()
        .scan(0, |sum, &mileage| {
            *sum += mileage;
            Some(*sum)
        })
        .take_while(|&sum| sum <= m)
        .count();

    println!("{count}");
}
