fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let input = buf.trim();
    const DIV_START: &str = r#"<div title=""#;
    const DIV_END: &str = r#"</div>"#;
    const P_START: &str = r#"<p>"#;
    const P_END: &str = r#"</p>"#;

    let mut cursor = 0;

    while let Some(div_start) = input[cursor..].find(DIV_START) {
        cursor += div_start + DIV_START.len();

        let i = input[cursor..].find('"').unwrap() + cursor;
        let title = &input[cursor..i];
        cursor = i + 1;

        println!("title : {title}");

        let div_end = input[cursor..].find(DIV_END).unwrap() + cursor;

        while let Some(p_start) = input[cursor..].find(P_START) {
            if p_start + cursor > div_end {
                break;
            }

            cursor += p_start + P_START.len();

            let p_end = input[cursor..].find(P_END).unwrap() + cursor;
            let paragraph = parse_paragraph(&input[cursor..p_end]);
            cursor = p_end + 1;

            println!("{paragraph}");
        }
    }
}

fn parse_paragraph(s: &str) -> String {
    let mut paragraph = String::new();
    let mut is_tag = false;

    for ch in s.chars() {
        match ch {
            '<' => is_tag = true,
            '>' => {
                is_tag = false;
                continue;
            }
            _ => (),
        }

        if is_tag {
            continue;
        }

        paragraph.push(ch);
    }

    while paragraph.contains("  ") {
        paragraph = paragraph.replace("  ", " ");
    }

    paragraph.trim().to_owned()
}
