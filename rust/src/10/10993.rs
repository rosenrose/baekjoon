use std::fmt::Write;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();

    print!("{}", star(n));
}

fn star(n: usize) -> String {
    if n == 1 {
        return "*".to_string();
    }

    let inner = star(n - 1);
    let inner: Vec<_> = inner.lines().collect();

    let inner_height = inner.len();
    let inner_width = if n % 2 == 0 {
        inner[inner_height - 1].len()
    } else {
        inner[0].len()
    };

    let height = inner_height * 2 + 1;
    let width = inner_width * 2 + 3;

    let blank = " ".repeat((width - 1) / 2);
    let horizontal = "*".repeat(width);

    let mut result = String::new();

    if n % 2 == 0 {
        writeln!(result, "{horizontal}").unwrap();

        for i in 0..height - 1 {
            match i {
                i if i < inner_height => writeln!(
                    result,
                    "{}*{}{}{}*",
                    &blank[..i + 1],
                    &blank[..inner_height - 1 - i],
                    inner[i],
                    &blank[..(inner_height - 1 - i) * 2]
                ),
                i if i < height - 2 => writeln!(
                    result,
                    "{}*{}*",
                    &blank[..i + 1],
                    &blank[..(height - 2 - i) * 2 - 1]
                ),
                _ => writeln!(result, "{blank}*"),
            }
            .unwrap();
        }
    } else {
        for i in 0..height - 1 {
            match i {
                0 => writeln!(result, "{blank}*"),
                i if i < inner_height => writeln!(
                    result,
                    "{}*{}*",
                    &blank[..blank.len() - i],
                    &blank[..i * 2 - 1]
                ),
                _ => writeln!(
                    result,
                    "{}*{}{}{}*",
                    &blank[..blank.len() - i],
                    &blank[..i - inner_height],
                    inner[i - inner_height],
                    &blank[..(i - inner_height) * 2]
                ),
            }
            .unwrap();
        }

        writeln!(result, "{horizontal}").unwrap();
    }

    result
}
