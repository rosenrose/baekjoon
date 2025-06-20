use std::io;

const MAX: usize = 99;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [m, n] = [(); 2].map(|_| input.next().unwrap());
    let nums_len = n - m + 1;
    let mut num_engs = [(); MAX].map(|_| Default::default());

    for num in m..=n {
        let eng: String = num
            .to_string()
            .chars()
            .map(|c| match c {
                '0' => "zero",
                '1' => "one",
                '2' => "two",
                '3' => "three",
                '4' => "four",
                '5' => "five",
                '6' => "six",
                '7' => "seven",
                '8' => "eight",
                '9' => "nine",
                _ => unreachable!(),
            })
            .collect();

        num_engs[num - m] = (num, eng);
    }

    num_engs[..nums_len].sort_by(|(_, eng1), (_, eng2)| eng1.cmp(eng2));

    for chunk in num_engs[..nums_len].chunks(10) {
        for (num, _) in chunk {
            print!("{num} ");
        }
        println!("");
    }
}
