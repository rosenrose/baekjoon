use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for input in buf.lines() {
        let (errors, dance_output) = get_errors_and_output(input);

        if errors.is_empty() {
            println!("form ok: {input}");
            continue;
        }

        if errors.len() == 1 {
            println!("form error {}: {dance_output}", errors[0]);
            continue;
        }

        let last_error = *errors.last().unwrap();
        let rest_errors = format!("{:?}", &errors[..errors.len() - 1]).replace(['[', ']'], "");

        println!("form errors {rest_errors} and {last_error}: {dance_output}");
    }
}

fn get_errors_and_output(input: &str) -> (Vec<usize>, String) {
    const DIP: &str = "dip";
    const JIGGLE: &str = "jiggle";
    const TWIRL: &str = "twirl";

    let mut rules = [true; 6];
    let mut output = Vec::new();
    let dances: Vec<_> = input.split(' ').collect();

    for i in 0..dances.len() {
        if dances[i] == DIP {
            if !((i >= 1 && dances[i - 1] == JIGGLE)
                || (i >= 2 && dances[i - 2] == JIGGLE)
                || (i <= dances.len() - 2 && dances[i + 1] == TWIRL))
            {
                rules[1] = false;
                output.push(dances[i].to_uppercase());
                continue;
            }
        }

        output.push(dances[i].to_owned());
    }

    rules[2] = input.ends_with("clap stomp clap");

    if input.contains(TWIRL) {
        rules[3] = input.contains("hop");
    }

    rules[4] = !input.starts_with(JIGGLE);

    rules[5] = input.contains(DIP);

    let errors: Vec<_> = rules
        .iter()
        .enumerate()
        .skip(1)
        .filter_map(|(i, b)| (!b).then_some(i))
        .collect();

    (errors, output.join(" "))
}
