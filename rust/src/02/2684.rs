use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for input in buf.lines().skip(1) {
        let input = input.as_bytes();
        let counts: Vec<_> = ["TTT", "TTH", "THT", "THH", "HTT", "HTH", "HHT", "HHH"]
            .iter()
            .map(|coin| input.windows(3).filter(|&w| w == coin.as_bytes()).count())
            .collect();

        println!("{}", vec_join(&counts, " "));
    }
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
