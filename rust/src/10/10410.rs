use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut output = String::new();

    for _ in 0..parse_int(input.next().unwrap()) {
        let [name, study_date, birth_date, courses] = [(); 4].map(|_| input.next().unwrap());
        let courses = parse_int(courses);

        let study_year = parse_int(study_date.split_once('/').unwrap().0);
        let birth_year = parse_int(birth_date.split_once('/').unwrap().0);

        if study_year >= 2010 || birth_year >= 1991 {
            writeln!(output, "{name} eligible").unwrap();
            continue;
        }

        if courses > 8 * 5 {
            writeln!(output, "{name} ineligible").unwrap();
            continue;
        }

        writeln!(output, "{name} coach petitions").unwrap();
    }

    print!("{output}");
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
