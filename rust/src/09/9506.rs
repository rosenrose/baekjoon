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
    let mut divisors = Vec::new();

    for i in 1..=(num as f64).sqrt() as i32 {
        if num % i != 0 {
            continue;
        }

        divisors.push(i);

        let n = num / i;

        if n != i {
            divisors.push(n);
        }
    }

    divisors
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
