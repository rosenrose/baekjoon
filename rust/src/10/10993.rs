use std::fmt::Write;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut output = String::new();
    let n: usize = buf.trim().parse().unwrap();
    let result = star(n);

    for r in result {
        writeln!(output, "{}", r.trim_end()).unwrap();
    }

    print!("{output}");
}

fn star(n: usize) -> Vec<String> {
    if n == 1 {
        return vec!["*".to_string()];
    }

    let inner = star(n - 1);
    let inner_height = inner.len();
    let inner_width = inner[0].len();

    let height = inner_height * 2 + 1;
    let width = inner_width * 2 + 3;

    let blank = " ".repeat(inner_width * 2 - 1);
    let horizontal = "*".repeat(width);
    let point = format!("{:^width$}", "*");

    let side = (1..height - 1).map(|i| {
        format!(
            "{}*{}*{}",
            &blank[..height - 1 - i],
            &blank[..i * 2 - 1],
            &blank[..height - 1 - i],
        )
    });

    let mut result: Vec<String>;
    let rest_width = (width - inner_width) / 2;

    if n % 2 == 0 {
        result = side.rev().collect();
        result.insert(0, horizontal);
        result.push(point);

        for i in 1..=inner_height {
            result[i] = [
                &result[i][..rest_width],
                &inner[i - 1][..],
                &result[i][width - rest_width..],
            ]
            .concat();
        }
    } else {
        result = side.collect();
        result.insert(0, point);
        result.push(horizontal);

        for i in inner_height..height - 1 {
            result[i] = [
                &result[i][..rest_width],
                &inner[i - inner_height][..],
                &result[i][width - rest_width..],
            ]
            .concat();
        }
    }

    result
}
