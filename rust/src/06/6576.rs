use std::fmt::Write;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let buf = io::read_to_string(io::stdin())?;
    let mut input = buf.lines();
    let mut output = String::new();

    let n: usize = input.next().unwrap().parse().unwrap();
    let quad_tree = input.next().unwrap();
    let mut map = vec![vec!['\0'; n]; n];

    fill_map(&mut map, quad_tree, n, 0, 0);

    writeln!(output, "#define quadtree_width {n}")?;
    writeln!(output, "#define quadtree_height {n}")?;
    writeln!(output, "static char quadtree_bits[] = {{")?;

    for row in map {
        for chunk in row.chunks(8) {
            let hex: u8 = chunk
                .iter()
                .enumerate()
                .map(|(i, &ch)| if ch == 'B' { 1 << i } else { 0 })
                .sum();

            write!(output, "0x{hex:02x},")?;
        }
        writeln!(output, "")?;
    }
    writeln!(output, "}};")?;

    print!("{output}");
    Ok(())
}

fn fill_map(map: &mut Vec<Vec<char>>, quad_tree: &str, n: usize, x: usize, y: usize) {
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
            _ => unreachable!(),
        }
    }
    // println!("{indices:?}");
    let left_top = &quad_tree[indices[0]..indices[1]];
    let right_top = &quad_tree[indices[1]..indices[2]];
    let left_bottom = &quad_tree[indices[2]..indices[3]];
    let right_bottom = &quad_tree[indices[3]..];
    let half = n >> 1;

    fill_map(map, left_top, half, x, y);
    fill_map(map, right_top, half, x + half, y);
    fill_map(map, left_bottom, half, x, y + half);
    fill_map(map, right_bottom, half, x + half, y + half);
}
