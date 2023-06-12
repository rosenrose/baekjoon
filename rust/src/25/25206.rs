use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let mut credit_sum = 0.0;
    let sum: f64 = (0..20)
        .filter_map(|_| {
            let [_, credit, grade] = [(); 3].map(|_| input.next().unwrap());
            let credit: f64 = credit.parse().unwrap();

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
                        _ => unreachable!(),
                    }
            })
        })
        .sum();

    println!("{}", sum / credit_sum);
}
