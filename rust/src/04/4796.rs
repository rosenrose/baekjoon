use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    for (i, [camping, max, vacation]) in (1..).map(|i| (i, [(); 3].map(|_| input.next().unwrap())))
    {
        if [camping, max, vacation] == [0; 3] {
            break;
        }

        let days = (vacation / max) * camping + (vacation % max).min(camping);

        writeln!(output, "Case {i}: {days}").unwrap();
    }

    print!("{output}");
}
