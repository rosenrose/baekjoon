use std::collections::HashMap;
use std::io;

const ZODIAC: [&str; 12] = [
    "Ox", "Tiger", "Rabbit", "Dragon", "Snake", "Horse", "Goat", "Monkey", "Rooster", "Dog", "Pig",
    "Rat",
];

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let n: i32 = input.next().unwrap().parse().unwrap();
    let cycle = ZODIAC.len() as i32;
    let mut birth_years = HashMap::from([("Bessie", ("Ox", 0))]);

    for _ in 0..n {
        let [name, .., order, animal, _, _, from] = [(); 8].map(|_| input.next().unwrap());
        let (from_animal, from_year) = birth_years[from];

        let from_idx = ZODIAC.iter().position(|&s| s == from_animal).unwrap() as i32;
        let idx = ZODIAC.iter().position(|&s| s == animal).unwrap() as i32;

        let birth_year = match order {
            "next" => {
                from_year
                    + if from_idx == idx {
                        cycle
                    } else {
                        (idx - from_idx + cycle) % cycle
                    }
            }
            "previous" => {
                from_year
                    - if from_idx == idx {
                        cycle
                    } else {
                        (from_idx - idx + cycle) % cycle
                    }
            }
            _ => unreachable!(),
        };

        birth_years.insert(name, (animal, birth_year));
    }
    // println!("{birth_years:?}");
    let diff_year = birth_years["Bessie"].1.abs_diff(birth_years["Elsie"].1);

    println!("{diff_year}");
}
