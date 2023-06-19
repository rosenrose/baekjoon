fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut input = buf.split_whitespace();
    let [a, b, c, d] = [(); 4].map(|_| input.next().unwrap());
    let [ab, cd] = [[a, b], [c, d]].map(|num| num.concat().parse::<i64>().unwrap());

    println!("{}", ab + cd);
}
