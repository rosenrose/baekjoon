use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<i64>);

    let mut memo = [0; 100];
    let coins = [25, 10, 1];

    for i in 1..100 {
        if i > 25 {
            memo[i] = (memo[i - 10] + memo[10]).min(memo[i - 25] + memo[25]);
            continue;
        }

        let (mut price, mut count) = (i, 0);

        for coin in coins {
            if coin > price {
                continue;
            }

            count += price / coin;
            price -= coin * (price / coin);
        }

        memo[i] = count;
    }

    for mut price in input.skip(1) {
        let mut count = 0;

        while price > 0 {
            count += memo[(price % 100) as usize];
            price /= 100;
        }

        println!("{count}")
    }
}
