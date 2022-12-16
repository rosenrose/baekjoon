use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf.lines().map(|s| s.parse::<i64>().unwrap());

    let mut cache = [0; 100];
    let coins = [25, 10, 1];

    for i in 1..100 {
        if i > 25 {
            cache[i] = (cache[i - 10] + cache[10]).min(cache[i - 25] + cache[25]);
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

        cache[i] = count;
    }

    for mut price in input.skip(1) {
        let mut count = 0;

        while price > 0 {
            count += cache[(price % 100) as usize];
            price /= 100;
        }

        println!("{count}")
    }
}
