use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();
    let [a, b] = [(); 2].map(|_| input.next().unwrap());

    let mut result = vec![String::new(); 5];

    a.chars().zip(b.chars()).for_each(|(a, b)| {
        result[0].push(if a == '1' && b == '1' { '1' } else { '0' });
        result[1].push(if a == '1' || b == '1' { '1' } else { '0' });
        result[2].push(if a != b { '1' } else { '0' });
        result[3].push(if a == '1' { '0' } else { '1' });
        result[4].push(if b == '1' { '0' } else { '1' });
    });

    println!("{}", result.join("\n"));
}
