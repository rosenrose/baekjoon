use std::io;

const MAX: usize = 9;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let (n, c) = input.next().unwrap().split_once(' ').unwrap();
    let (n, c) = (parse_int(n), parse_int(c));
    let mut orders = [(); MAX].map(|_| Vec::new());

    for (i, line) in input.by_ref().take(n).enumerate() {
        orders[i] = line.split(' ').map(parse_int).skip(1).collect();
    }

    let mut cards = [(); MAX].map(|_| Vec::new());

    for (i, line) in input.enumerate() {
        cards[i] = line
            .split(',')
            .map(|card| {
                let (action, arg) = card.split_once(' ').unwrap();
                (action, arg.chars().nth(0).unwrap())
            })
            .collect();
    }
    // println!("{orders:?}\n{cards:?}");
    let mut result = Vec::new();

    product(
        &mut [0; MAX][..n],
        &mut [0; MAX][..c],
        &orders[..n],
        &cards[..c],
        &mut result,
    );
    result.sort();
    result.dedup();

    println!("{}", result.join("\n"));
}

fn product(
    depths: &mut [usize],
    selected: &mut [usize],
    orders: &[Vec<usize>],
    cards: &[Vec<(&str, char)>],
    result: &mut Vec<String>,
) {
    let total_depth: usize = depths.iter().sum();

    if total_depth == selected.len() {
        let mut ret = String::new();
        // println!("{selected:?}");
        for i in selected {
            for &(action, arg) in &cards[*i - 1] {
                match action {
                    "ADD" => ret.push(arg),
                    "DEL" => {
                        let idx = (arg as u8 - b'0') as usize;

                        if idx >= ret.len() {
                            result.push("ERROR".to_owned());
                            return;
                        }

                        ret.remove(idx);
                    }
                    _ => unreachable!(),
                }
            }
        }

        result.push(if ret.is_empty() {
            "EMPTY".to_owned()
        } else {
            ret
        });

        return;
    }

    for (i, order) in orders.iter().enumerate() {
        let Some(&card_num) = order.get(depths[i]) else {
            continue;
        };

        selected[total_depth] = card_num;
        depths[i] += 1;

        product(depths, selected, orders, cards, result);

        depths[i] -= 1;
    }
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
