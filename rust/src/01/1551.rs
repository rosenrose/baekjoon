use std::string::ToString;

fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    if let [_, k] = parse_int_vec(&buf)[..] {
        read_line(&mut buf);

        if k == 0 {
            println!("{buf}");
            return;
        }

        let arr: Vec<i32> = buf.trim().split(',').map(|s| s.parse().unwrap()).collect();

        let mut new_arr: Vec<i32> = (0..arr.len() - 1).map(|i| arr[i + 1] - arr[i]).collect();

        for _ in 0..k - 1 {
            new_arr = (0..new_arr.len() - 1)
                .map(|i| new_arr[i + 1] - new_arr[i])
                .collect();
        }

        println!("{}", vec_join(&new_arr, ","))
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
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
