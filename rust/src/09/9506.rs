use std::collections::BTreeSet;
use std::string::ToString;

fn main() {
    let mut buf = String::new();

    loop {
        read_line(&mut buf);
        let num = parse_int(&buf);

        if num == -1 {
            return;
        }

        let divisors = (1..)
            .take_while(|i| i * i <= num)
            .fold(BTreeSet::new(), |mut acc, i| {
                if num % i != 0 {
                    return acc;
                }

                if i != num {
                    acc.insert(i);
                }
                if i != 1 {
                    acc.insert(num / i);
                }

                acc
            });

        let sum: i32 = divisors.iter().sum();

        if sum == num {
            println!("{num} = {}", set_join(&divisors, " + "));
            continue;
        }

        println!("{num} is NOT perfect.");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}

fn set_join<T>(vec: &BTreeSet<T>, seperator: &str) -> String
where
    T: ToString,
{
    vec.iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join(seperator)
}
