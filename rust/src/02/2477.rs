use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let k = input.next().unwrap();
    let mut longest = 0;
    let mut lengths = [0; 6];

    for length in &mut lengths {
        let [_, num] = [(); 2].map(|_| input.next().unwrap());
        longest = num.max(longest);
        *length = num;
    }

    while lengths[0] != longest {
        lengths.rotate_left(1);
    }

    let (outer_area, inner_area) = if lengths[1] < lengths[5] {
        (lengths[0] * lengths[5], lengths[2] * lengths[3])
    } else {
        (lengths[0] * lengths[1], lengths[3] * lengths[4])
    };

    println!("{}", (outer_area - inner_area) * k);
}
