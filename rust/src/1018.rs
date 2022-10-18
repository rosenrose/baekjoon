fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    if let [n, m] = parse_int_vec(&buf)[..] {
        let board: Vec<String> = (0..n)
            .map(|_| {
                read_line(&mut buf);
                buf.trim().to_string()
            })
            .collect();

        let mut min = 32;

        for i in 0..=n - 8 {
            for j in 0..=m - 8 {
                let paint = get_paint_count(&board, i as usize, j as usize);

                if paint < min {
                    min = paint;
                }
            }
        }

        println!("{min}");
    }
}

fn get_paint_count(board: &Vec<String>, i: usize, j: usize) -> i32 {
    let mut paint_start_from_black = 0;

    for y in 0..8 {
        for (x, bw) in board[y + i].chars().skip(j).take(8).enumerate() {
            match (y % 2, x % 2) {
                (0, 0) => {
                    if bw == 'W' {
                        paint_start_from_black += 1;
                    }
                }
                (0, 1) => {
                    if bw == 'B' {
                        paint_start_from_black += 1;
                    }
                }
                (1, 0) => {
                    if bw == 'B' {
                        paint_start_from_black += 1;
                    }
                }
                (1, 1) => {
                    if bw == 'W' {
                        paint_start_from_black += 1;
                    }
                }
                _ => (),
            }
        }
    }

    let paint_start_from_white = 64 - paint_start_from_black;

    paint_start_from_black.min(paint_start_from_white)
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

/*
BBBWBWBW     BWBWBWBW
BBBBWBWB     WBWBWBWB
BBBWBWBW     BWBWBWBW
BBBBWBWB     WBWBWBWB
BBBWBWBW  -> BWBWBWBW
BBBBWBWB     WBWBWBWB
WWWWWBWB     BWBWBWBW
WWWWWBWB     WBWBWBWB
paint: 17

BBBWBWBW     WBWBWBWB
BBBBWBWB     BWBWBWBW
BBBWBWBW     WBWBWBWB
BBBBWBWB     BWBWBWBW
BBBWBWBW  -> WBWBWBWB
BBBBWBWB     BWBWBWBW
WWWWWBWB     WBWBWBWB
WWWWWBWB     BWBWBWBW
paint: 47
*/
