use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let mut tree = vec![[None; 2]; n + 1];
    let mut parents = vec![0; n + 1];

    for [node, children @ ..] in (0..n).map(|_| [(); 3].map(|_| input.next().unwrap())) {
        for (i, &child) in children.iter().enumerate() {
            if child == -1 {
                continue;
            }

            tree[node as usize][i] = Some(child as usize);
            parents[child as usize] = node as usize;
        }
    }

    let root = parents[1..].iter().position(|&p| p == 0).unwrap() + 1;
    let mut tree_widths = vec![0; n + 1];
    tree_widths[root] = get_tree_width(&tree, root, &mut tree_widths);

    let mut cols = vec![0; n + 1];
    get_cols(&tree, root, &mut cols, root, &tree_widths, &parents);

    let level_widths = bfs(&tree, root, &cols);
    let (mut max_width, mut max_width_level) = (0, 0);
    // println!("{tree_widths:?}\n{cols:?}\n{level_widths:?}");
    for (i, &width) in level_widths.iter().enumerate() {
        if max_width < width {
            max_width = width;
            max_width_level = i;
        }
    }

    println!("{max_width_level} {max_width}");
}

fn get_tree_width(
    tree: &[[Option<usize>; 2]],
    start: usize,
    tree_widths: &mut Vec<usize>,
) -> usize {
    let left_tree_width = if let Some(left) = tree[start][0] {
        let ret = get_tree_width(tree, left, tree_widths);
        tree_widths[left] = ret;

        ret
    } else {
        0
    };
    let right_tree_width = if let Some(right) = tree[start][1] {
        let ret = get_tree_width(tree, right, tree_widths);
        tree_widths[right] = ret;

        ret
    } else {
        0
    };

    left_tree_width + right_tree_width + 1
}

fn get_cols(
    tree: &[[Option<usize>; 2]],
    start: usize,
    cols: &mut Vec<usize>,
    root: usize,
    tree_widths: &[usize],
    parents: &[usize],
) {
    let left_tree_width = if let Some(left) = tree[start][0] {
        get_cols(tree, left, cols, root, tree_widths, parents);
        tree_widths[left]
    } else {
        0
    };

    let mut parent = parents[start];

    while !(cols[parent] != 0 || parent == 0) {
        parent = parents[parent];
    }

    cols[start] = cols[parent] + left_tree_width + 1;

    if let Some(right) = tree[start][1] {
        get_cols(tree, right, cols, root, tree_widths, parents);
    }
}

fn bfs(tree: &[[Option<usize>; 2]], root: usize, cols: &[usize]) -> Vec<usize> {
    let mut level_widths = vec![0; tree.len()];
    let mut prev_level = 1;
    let (mut min_col, mut max_col) = (usize::MAX, 0);
    let mut queue = VecDeque::from([(root, 1)]);

    while let Some((node, level)) = queue.pop_front() {
        if prev_level != level {
            level_widths[prev_level] = max_col - min_col + 1;
            (min_col, max_col) = (usize::MAX, 0);
        }

        prev_level = level;
        (min_col, max_col) = (cols[node].min(min_col), cols[node].max(max_col));

        for &child in tree[node].iter().flatten() {
            queue.push_back((child, level + 1));
        }
    }

    level_widths[prev_level] = max_col - min_col + 1;
    level_widths
}
