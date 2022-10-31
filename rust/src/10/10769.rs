use std::cmp::Ordering;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let input = buf.trim();

    let happy = input.matches(":-)").count();
    let sad = input.matches(":-(").count();

    if (happy, sad) == (0, 0) {
        println!("none");
        return;
    }

    match happy.cmp(&sad) {
        Ordering::Greater => println!("happy"),
        Ordering::Equal => println!("unsure"),
        Ordering::Less => println!("sad"),
    };
}
