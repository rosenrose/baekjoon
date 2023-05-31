use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let matrix: Vec<Vec<_>> = (0..n).map(|_| input.by_ref().take(n).collect()).collect();

    let min_diff = (1..=n / 2)
        .map(|i| combinations(0, 0, 0, i, n, &matrix))
        .min()
        .unwrap();

    println!("{min_diff}");
}

fn combinations(
    depth: usize,
    start: usize,
    mut selected: i32,
    choose: usize,
    numbers: usize,
    matrix: &[Vec<i32>],
) -> u32 {
    if depth == choose {
        let mut rest = !selected;
        let (mut start_score, mut link_score) = (0, 0);
        // println!("{selected:b} {rest:b}");
        if choose > 1 {
            let mut selected_nums = Vec::with_capacity(choose);

            for i in 0..numbers {
                if selected & 1 == 1 {
                    selected_nums.push(i);
                }

                selected >>= 1;
            }

            for i in 0..selected_nums.len() - 1 {
                for j in i + 1..selected_nums.len() {
                    let (a, b) = (selected_nums[i], selected_nums[j]);
                    start_score += matrix[a][b] + matrix[b][a];
                }
            }
        }

        let mut rest_nums = Vec::with_capacity(numbers - choose);

        for i in 0..numbers {
            if rest & 1 == 1 {
                rest_nums.push(i);
            }

            rest >>= 1;
        }

        for i in 0..rest_nums.len() - 1 {
            for j in i + 1..rest_nums.len() {
                let (a, b) = (rest_nums[i], rest_nums[j]);
                link_score += matrix[a][b] + matrix[b][a];
            }
        }

        return start_score.abs_diff(link_score);
    }

    let takes = numbers - (choose - 1);

    (start..depth + takes)
        .map(|num| {
            combinations(
                depth + 1,
                num + 1,
                selected | (1 << num),
                choose,
                numbers,
                matrix,
            )
        })
        .min()
        .unwrap()
}
