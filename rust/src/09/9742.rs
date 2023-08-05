use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    while let [Some(word), Some(order)] = [(); 2].map(|_| input.next()) {
        print!("{word} {order} = ");

        let mut word = word.as_bytes().to_vec();
        let mut order: i32 = order.parse().unwrap();

        while order > 1 {
            if next_permuation(&mut word) {
                order -= 1;
            } else {
                break;
            }
        }

        if order == 1 {
            println!("{}", String::from_utf8(word).unwrap());
        } else {
            println!("No permutation");
        }
    }
}

fn next_permuation(chars: &mut Vec<u8>) -> bool {
    let len = chars.len();

    let Some(i) = (1..len).rfind(|&i| chars[i - 1] < chars[i]) else {
        return false;
    };
    let j = (i..len).rfind(|&j| chars[j] > chars[i - 1]).unwrap();

    chars.swap(i - 1, j);
    chars[i..].sort();

    true
}
