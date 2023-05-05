use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    const INIT: usize = 100;
    let n = input.next().unwrap();
    let mut is_broken = [false; 10];

    for btn in input.skip(1) {
        is_broken[btn] = true;
    }

    if is_broken.iter().all(|&b| b) {
        println!("{}", n.abs_diff(INIT));
        return;
    }

    let dist_to_channel = |stopby: usize| {
        (if matches!(stopby, 98..=103) {
            stopby.abs_diff(INIT)
        } else {
            stopby.to_string().len()
        }) + n.abs_diff(stopby)
    };

    let (mut lower, mut upper) = (None, 0);

    'outer: for channel in (0..=n).rev() {
        if channel == INIT {
            lower = Some(channel);
            break;
        }
        if channel == 0 {
            lower = (!is_broken[0]).then_some(channel);
            break;
        }

        let mut num = channel;

        while num > 0 {
            if is_broken[(num % 10) as usize] {
                continue 'outer;
            }

            num /= 10;
        }

        lower = Some(channel);
        break;
    }

    if is_broken[1..].iter().all(|&b| b) {
        // upper 불가능
        println!("{}", dist_to_channel(lower.unwrap()));
        return;
    }

    'outer: for channel in n + 1.. {
        if channel == INIT {
            upper = channel;
            break;
        }

        let mut num = channel;

        while num > 0 {
            if is_broken[(num % 10) as usize] {
                continue 'outer;
            }

            num /= 10;
        }

        upper = channel;
        break;
    }
    // println!("{lower} {upper}");
    println!(
        "{}",
        if let Some(lower) = lower {
            dist_to_channel(lower).min(dist_to_channel(upper))
        } else {
            dist_to_channel(upper)
        }
    );
}
