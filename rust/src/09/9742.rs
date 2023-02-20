use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    while let (Some(word), Some(order)) = (input.next(), input.next()) {
        print!("{word} {order} = ");

        let mut word = word.as_bytes().to_vec();
        let len = word.len();
        let mut order: i32 = order.parse().unwrap();

        while order > 1 {
            let Some(i) = (1..len).rfind(|&i| word[i - 1] < word[i]) else {
                break;
            };
            let j = (i..len).rfind(|&j| word[j] > word[i - 1]).unwrap();

            word.swap(i - 1, j);
            (&mut word[i..]).sort();

            order -= 1;
        }

        if order == 1 {
            println!("{}", String::from_utf8(word).unwrap());
        } else {
            println!("No permutation");
        }
    }
}
