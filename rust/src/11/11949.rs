use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [n, m] = [(); 2].map(|_| input.next().unwrap());
    let mut students: Vec<_> = input.by_ref().take(n).collect();

    for i in 1..=m {
        for j in 1..n {
            if students[j - 1] % i > students[j] % i {
                students.swap(j - 1, j);
            }
        }
    }

    for num in students {
        println!("{num}");
    }
}
