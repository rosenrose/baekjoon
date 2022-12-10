use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let angles: Vec<_> = buf.lines().map(|s| s.parse::<i32>().unwrap()).collect();

    if angles.iter().sum::<i32>() != 180 {
        println!("Error");
        return;
    }

    if angles.iter().all(|&angle| angle == 60) {
        println!("Equilateral");
        return;
    }

    if angles[0] == angles[1] || angles[0] == angles[2] || angles[1] == angles[2] {
        println!("Isosceles");
        return;
    }

    println!("Scalene");
}
