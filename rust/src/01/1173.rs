use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [n, min, max, inc, dec] = [(); 5].map(|_| input.next().unwrap());

    if inc > (max - min) {
        println!("-1");
        return;
    }

    let mut current = min;
    let (mut exercise, mut time) = (0, 0);

    'outer: loop {
        while (max - current) >= inc {
            current += inc;
            exercise += 1;
            time += 1;

            if exercise >= n {
                break 'outer;
            }
        }

        while (max - current) < inc {
            current = min.max(current - dec);
            time += 1;
        }
    }

    println!("{time}");
}
