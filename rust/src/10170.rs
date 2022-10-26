use std::fmt::Display;

fn main() {
    let west = [
        ("Seattle", 13, 3, 0),
        ("San Francisco", 12, 4, 0),
        ("Arizona", 10, 6, 0),
        ("St. Louis", 7, 9, 0),
    ];

    let north = [
        ("Green Bay", 8, 7, 1),
        ("Chicago", 8, 8, 0),
        ("Detroit", 7, 9, 0),
        ("Minnesota", 5, 10, 1),
    ];

    print_nfc("West", &west);
    println!("");
    print_nfc("North", &north);
}

fn print_nfc(title: &str, data: &[(&str, i32, i32, i32)]) {
    let col_width = [14, 2, 2, 2];
    let title_row = (format!("NFC {title}"), "W", "L", "T");

    print_nfc_row(&col_width, title_row);

    println!(
        "{}",
        "-".repeat(col_width.iter().sum::<usize>() + col_width.len() - 1)
    );

    for &row in data {
        print_nfc_row(&col_width, row);
    }
}

fn print_nfc_row<T, U>(col_width: &[usize], row: (T, U, U, U))
where
    T: Display,
    U: Display,
{
    println!(
        "{:w0$} {:<w1$} {:>w2$} {:>w3$}",
        row.0,
        row.1,
        row.2,
        row.3,
        w0 = col_width[0],
        w1 = col_width[1],
        w2 = col_width[2],
        w3 = col_width[3]
    );
}
