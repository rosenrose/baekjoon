fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let input = buf.trim();

    for (div_start, div_prefix) in input.match_indices(r#"<div title=""#) {
        let div_end = input[div_start..].find("</div>").unwrap() + div_start;

        let quote_start = div_start + div_prefix.len();
        let quote_end = input[quote_start..].find('"').unwrap() + quote_start;

        let title = &input[quote_start..quote_end];
        println!("title : {title}");

        for (p_start, p_prefix) in input[quote_end..div_end].match_indices("<p>") {
            let p_start = p_start + quote_end;
            let p_end = input[p_start..].find("</p>").unwrap() + p_start;

            let paragraph = parse_paragraph(&input[p_start + p_prefix.len()..p_end]);
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
