use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let regex = Regex::new(r#"^"([A-Z ]+)" \1$"#, false);

    for input in buf.lines().take_while(|&input| input != "END") {
        if regex.find(input).is_some() {
            println!("Quine({})", &input[input.rfind('"').unwrap() + 2..]);
        } else {
            println!("not a quine");
        }
    }

    // const NOT: &str = "not a quine";

    // for input in buf.lines().take_while(|&input| input != "END") {
    //     let (Some(start @ 0), Some(end)) = (input.find('"'), input.rfind('"')) else {
    //         println!("{NOT}");
    //         continue;
    //     };

    //     if end - start <= 1 {
    //         println!("{NOT}");
    //         continue;
    //     }

    //     if input.chars().nth(end + 1) != Some(' ') {
    //         println!("{NOT}");
    //         continue;
    //     }

    //     let phrase = &input[start + 1..end];

    //     if phrase.chars().any(|ch| !matches!(ch, 'A'..='Z' | ' ')) {
    //         println!("{NOT}");
    //         continue;
    //     }

    //     if &input[(end + 2).min(input.len() - 1)..] != phrase {
    //         println!("{NOT}");
    //         continue;
    //     }

    //     println!("Quine({phrase})");
    // }
}
