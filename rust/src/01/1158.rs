use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [n, k] = [(); 2].map(|_| input.next().unwrap());
    let mut circle: Vec<_> = (1..=n).collect();
    let mut index = 0;
    let mut josephus_permutation = Vec::new();

    while !circle.is_empty() {
        index = (index + k - 1) % circle.len();
        josephus_permutation.push(circle.remove(index));
    }

    println!(
        "<{}>",
        format!("{josephus_permutation:?}").replace(['[', ']'], "")
    );
}
