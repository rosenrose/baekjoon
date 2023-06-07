use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    const INIT: usize = 100;
    let target = input.next().unwrap();
    let mut is_broken = [false; 10];

    for btn in input.skip(1) {
        is_broken[btn] = true;
    }

    if is_broken.iter().all(|&b| b) {
        println!("{}", target.abs_diff(INIT));
        return;
    }

    let dist_to_target = |stopby: usize| {
        target.abs_diff(stopby)
            + if matches!(stopby, 98..=103) {
                stopby.abs_diff(INIT)
            } else {
                stopby.to_string().len()
            }
    };

    let lower = 'a: {
        'outer: for channel in (0..=target).rev() {
            if channel == INIT {
                break 'a Some(channel);
            }
            if channel == 0 {
                break 'a (!is_broken[0]).then_some(channel);
            }

            let mut digits = channel;

            while digits > 0 {
                if is_broken[digits % 10] {
                    continue 'outer;
                }

                digits /= 10;
            }

            break 'a Some(channel);
        }

        None
    };

    if is_broken[1..].iter().all(|&b| b) {
        // upper 불가능
        println!("{}", dist_to_target(lower.unwrap()));
        return;
    }

    let upper = 'a: {
        'outer: for channel in target + 1.. {
            if channel == INIT {
                break 'a channel;
            }

            let mut digits = channel;

            while digits > 0 {
                if is_broken[digits % 10] {
                    continue 'outer;
                }

                digits /= 10;
            }

            break 'a channel;
        }

        unreachable!()
    };
    // println!("{lower} {upper}");
    println!(
        "{}",
        if let Some(lower) = lower {
            dist_to_target(lower).min(dist_to_target(upper))
        } else {
            dist_to_target(upper)
        }
    );
}
