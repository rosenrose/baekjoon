use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    const NOT: &str = "not a quine";

    for input in buf.lines().take_while(|&input| input != "END") {
        let Some(start) = input.find('"') else {
            println!("{NOT}");
            continue;
        };
        let Some(end) = input.rfind('"') else {
            println!("{NOT}");
            continue;
        };

        if start != 0 {
            println!("{NOT}");
            continue;
        }
        if end - start <= 1 {
            println!("{NOT}");
            continue;
        }

        let phrase = &input[start + 1..end];

        if !phrase.chars().all(|ch| matches!(ch, 'A'..='Z' | ' ')) {
            println!("{NOT}");
            continue;
        }

        if input.chars().nth(end + 1) != Some(' ') {
            println!("{NOT}");
            continue;
        }
        if &input[(end + 2).min(input.len() - 1)..] != phrase {
            println!("{NOT}");
            continue;
        }

        println!("Quine({phrase})");
    }
}
