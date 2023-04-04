use std::fmt::Write;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut input = buf.trim().to_owned();
    let mut output = String::new();

    while input.contains("  ") {
        input = input.replace("  ", " ");
    }

    let mut delim_indices: Vec<_> = input
        .match_indices(['<', '>', '(', ')'])
        .chain(
            ["&&", "||"]
                .iter()
                .flat_map(|delim| input.match_indices(delim)),
        )
        .collect();
    delim_indices.sort();
    // println!("{input}\n{delim_indices:?}");
    let mut cursor = 0;

    for (delim_start, delim) in delim_indices {
        let token = input[cursor..delim_start].trim();
        let delim_end = delim_start + delim.len();

        (if token.is_empty() {
            write!(output, "{delim} ")
        } else {
            write!(output, "{token} {delim} ")
        })
        .unwrap();

        cursor = delim_end;
    }

    if cursor < input.len() {
        write!(output, "{}", &input[cursor..].trim()).unwrap();
    }

    print!("{}", output.trim());
}
