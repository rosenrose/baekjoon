use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();
    let mut output = String::new();

    let n: usize = input.next().unwrap().parse().unwrap();
    let quad_tree = input.next().unwrap();
    let mut map = vec![vec!['\0'; n]; n];

    fill_map(&mut map, quad_tree, 0, 0, n);

    writeln!(output, "#define quadtree_width {n}").unwrap();
    writeln!(output, "#define quadtree_height {n}").unwrap();
    writeln!(output, "static char quadtree_bits[] = {{").unwrap();

    for row in map {
        for chunk in row.chunks(8) {
            let hex: u8 = chunk
                .iter()
                .enumerate()
                .map(|(i, &ch)| if ch == 'B' { 1 << i } else { 0 })
                .sum();

            write!(output, "0x{hex:02x},").unwrap();
        }
        writeln!(output, "").unwrap();
    }
    writeln!(output, "}};").unwrap();

    print!("{output}");
}

fn fill_map(map: &mut Vec<Vec<char>>, quad_tree: &str, x: usize, y: usize, n: usize) {
    if quad_tree.len() == 1 {
        let ch = quad_tree.chars().nth(0).unwrap();

        for row in map[y..y + n].iter_mut() {
            for cell in row[x..x + n].iter_mut() {
                *cell = ch;
            }
        }

        return;
    }

    let mut indices = Vec::with_capacity(4);
    let mut depth = 0;

    for (i, ch) in quad_tree.char_indices().skip(1) {
        match ch {
            'Q' => {
                if depth == 0 {
                    indices.push(i);
                }

                depth += if depth == 0 { 4 } else { 3 };
            }
            'B' | 'W' => {
                if depth == 0 {
                    indices.push(i);
                } else {
                    depth -= 1;
                }
            }
            _ => (),
        }
    }
    // println!("{indices:?}");
    let left_top = &quad_tree[indices[0]..indices[1]];
    let right_top = &quad_tree[indices[1]..indices[2]];
    let left_bottom = &quad_tree[indices[2]..indices[3]];
    let right_bottom = &quad_tree[indices[3]..];
    let half = n / 2;

    fill_map(map, left_top, x, y, half);
    fill_map(map, right_top, x + half, y, half);
    fill_map(map, left_bottom, x, y + half, half);
    fill_map(map, right_bottom, x + half, y + half, half);
}
