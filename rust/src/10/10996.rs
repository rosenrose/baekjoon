use std::fmt::Write;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();

    if n == 1 {
        println!("*");
        return;
    }

    let mut output = String::new();

    for i in 1..=n * 2 {
        let star = if n % 2 == 0 {
            (if i % 2 == 0 { " *" } else { "* " }).repeat(n / 2)
        } else {
            if i % 2 == 0 {
                " *".repeat(n / 2)
            } else {
                "* ".repeat((n + 1) / 2)
            }
        };

        writeln!(output, "{}", star.trim_end()).unwrap();
    }

    print!("{output}");
}
