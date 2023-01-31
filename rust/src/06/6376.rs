fn main() {
    println!("n e");
    println!("- -----------");

    for n in 0..=9 {
        let e = 1.0
            + (1..=n)
                .map(|i| ((1..=i).product::<i32>() as f64).recip())
                .sum::<f64>();

        if e.to_string().len() > 11 {
            println!("{n} {e:.9}");
        } else {
            println!("{n} {e}");
        }
    }
}
