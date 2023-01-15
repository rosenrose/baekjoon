fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n, k] = parse_int_vec(&buf)[..] else { return };
    let mut circle: Vec<_> = (1..=n).collect();
    let mut index = 0;
    let mut josephus_permutation = Vec::new();

    while !circle.is_empty() {
        index = (index + k - 1) % circle.len();
        josephus_permutation.push(circle.remove(index));
    }

    println!("<{}>", vec_join(&josephus_permutation, ", "));
}

fn parse_int_vec(buf: &String) -> Vec<usize> {
    buf.split_whitespace().flat_map(str::parse).collect()
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
