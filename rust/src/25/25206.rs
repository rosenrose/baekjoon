use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    let mut credit_sum = 0.0;
    let sum: f64 = (0..20)
        .filter_map(|_| {
            let (_, credit, grade) = (input(), input().parse::<f64>().unwrap(), input());

            (grade != "P").then(|| {
                credit_sum += credit;

                credit
                    * match grade {
                        "A+" => 4.5,
                        "A0" => 4.0,
                        "B+" => 3.5,
                        "B0" => 3.0,
                        "C+" => 2.5,
                        "C0" => 2.0,
                        "D+" => 1.5,
                        "D0" => 1.0,
                        "F" => 0.0,
                        _ => Default::default(),
                    }
            })
        })
        .sum();

    println!("{}", sum / credit_sum);
}
