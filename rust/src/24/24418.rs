use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i128>);

    let n = input.next().unwrap();
    // let matrix: Vec<Vec<_>> = (0..n).map(|_| input.by_ref().take(n).collect()).collect();
    // let mut memo = vec![vec![0; n + 1]; n + 1];
    // matrix_path_recursive(&matrix, n, n, &mut count_1);
    // matrix_path_dp(&matrix, n, n, &mut memo, &mut count_2);

    let count_1 = (n + 1..=2 * n).product::<i128>() / (1..=n).product::<i128>();
    let count_2 = n * n;
    // https://ko.wikipedia.org/wiki/중심이항계수
    println!("{count_1} {count_2}");
}

// fn matrix_path_recursive(matrix: &[Vec<i32>], r: usize, c: usize, count: &mut i32) -> i32 {
//     if r == 0 || c == 0 {
//         *count += 1;
//         return 0;
//     }

//     let up = matrix_path_recursive(matrix, r - 1, c, count);
//     let left = matrix_path_recursive(matrix, r, c - 1, count);

//     matrix[r - 1][c - 1] + up.max(left)
// }

// fn matrix_path_dp(
//     matrix: &[Vec<i32>],
//     r: usize,
//     c: usize,
//     memo: &mut Vec<Vec<i32>>,
//     count: &mut i32,
// ) -> i32 {
//     for i in 1..=r {
//         for j in 1..=c {
//             *count += 1;
//             memo[i][j] = matrix[i - 1][j - 1] + memo[i - 1][j].max(memo[i][j - 1]);
//         }
//     }

//     memo[r][c]
// }
