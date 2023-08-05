use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut output = String::new();

    let [n, _] = [(); 2].map(|_| input.next().unwrap().parse::<i32>().unwrap());

    let mut files: Vec<_> = (0..n)
        .flat_map(|_| input.next().unwrap().split_once('.'))
        .collect();
    let allowed_exts: HashSet<_> = input.collect();

    files.sort_unstable_by(|(a_name, a_ext), (b_name, b_ext)| {
        a_name.cmp(b_name).then_with(|| {
            match (allowed_exts.contains(a_ext), allowed_exts.contains(b_ext)) {
                (true, true) | (false, false) => a_ext.cmp(b_ext),
                (true, false) => Ordering::Less,
                (false, true) => Ordering::Greater,
            }
        })
    });

    for (stem, ext) in files {
        writeln!(output, "{stem}.{ext}").unwrap();
    }

    print!("{output}");
}
