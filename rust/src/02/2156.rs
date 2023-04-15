use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<i32>);

    let wines: Vec<_> = input.skip(1).collect();

    match wines.len() {
        1 => println!("{}", wines[0]),
        2 => println!("{}", wines[0] + wines[1]),
        _ => {
            let mut max_wines = vec![0; wines.len()];
            max_wines[0] = wines[0];
            max_wines[1] = wines[0] + wines[1];
            max_wines[2] = (wines[0].max(wines[1]) + wines[2]).max(wines[0] + wines[1]);

            for i in 3..wines.len() {
                let drink_last = max_wines[i - 2].max(max_wines[i - 3] + wines[i - 1]) + wines[i];
                let not_drink_last = max_wines[i - 1];

                max_wines[i] = drink_last.max(not_drink_last);
            }
            // println!("{max_wines:?}");
            println!("{}", max_wines.last().unwrap());
        }
    }
}
