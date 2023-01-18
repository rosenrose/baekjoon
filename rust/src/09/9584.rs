use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();
    let mut output = String::new();

    let code = input.next().unwrap();
    let regex = Regex::new(&format!("^{}$", code.replace('*', ".")), false);
    let mut count = 0;

    for reg in input.skip(1).filter(|reg| regex.find(reg).is_some()) {
        count += 1;
        writeln!(output, "{reg}").unwrap();
    }

    print!("{count}\n{output}");
}
