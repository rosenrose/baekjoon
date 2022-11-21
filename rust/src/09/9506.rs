use std::string::ToString;

fn main() {
    let mut buf = String::new();

    loop {
        read_line(&mut buf);
        let num = parse_int(&buf);

        if num == -1 {
            return;
        }

        let mut divisors = get_divisors(num);
        divisors.sort();
        divisors.pop();

        let sum: i32 = divisors.iter().sum();

        if sum == num {
            println!("{num} = {}", vec_join(&divisors, " + "));
            continue;
        }

        println!("{num} is NOT perfect.");
    }
}

fn get_divisors(num: i32) -> Vec<i32> {
    (1..)
        .take_while(|i| i * i <= num)
        .filter_map(|i| {
            if num % i == 0 {
                let mut divisor = vec![i, num / i];
                divisor.dedup();

                Some(divisor)
            } else {
                None
            }
        })
        .flatten()
        .collect()
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}

fn vec_join<T>(vec: &Vec<T>, seperator: &str) -> String
where
    T: ToString,
{
    vec.iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join(seperator)
}
