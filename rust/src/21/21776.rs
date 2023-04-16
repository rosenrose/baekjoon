use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let (n, c) = input.next().unwrap().split_once(' ').unwrap();
    let (n, c) = (parse_int(n), parse_int(c));

    let card_orders: Vec<Vec<_>> = input
        .by_ref()
        .take(n)
        .map(|line| line.split(' ').map(parse_int).skip(1).collect())
        .collect();
    let cards: Vec<Vec<_>> = input
        .map(|line| {
            line.split(',')
                .map(|card| {
                    let (action, param) = card.split_once(' ').unwrap();
                    (action, param.chars().nth(0).unwrap())
                })
                .collect()
        })
        .collect();
    // println!("{card_orders:?}\n{cards:?}");
    let mut result = Vec::new();
    product(
        &mut vec![0; n],
        &mut vec![0; c],
        &card_orders,
        &cards,
        &mut result,
    );
    result.sort();
    result.dedup();

    println!("{}", result.join("\n"));
}

fn product(
    depths: &mut Vec<usize>,
    selected: &mut Vec<usize>,
    card_orders: &Vec<Vec<usize>>,
    cards: &Vec<Vec<(&str, char)>>,
    result: &mut Vec<String>,
) {
    let total_depth: usize = depths.iter().sum();

    if total_depth == selected.len() {
        let mut ret = String::new();
        // println!("{selected:?}");
        for &i in selected.iter() {
            for &(action, param) in &cards[i - 1] {
                match action {
                    "ADD" => ret.push(param),
                    "DEL" => {
                        let idx = param as usize - '0' as usize;

                        if idx >= ret.len() {
                            result.push("ERROR".to_owned());
                            return;
                        }

                        ret.remove(idx);
                    }
                    _ => (),
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

    for (i, orders) in card_orders.iter().enumerate() {
        let Some(&card_num) = orders.get(depths[i]) else {
            continue;
        };

        selected[total_depth] = card_num;
        depths[i] += 1;

        product(depths, selected, card_orders, cards, result);

        depths[i] -= 1;
    }
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
