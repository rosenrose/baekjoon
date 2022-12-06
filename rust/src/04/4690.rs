fn main() {
    for a in 6..=100_i32 {
        let a3 = a * a * a;

        for b in (2_i32..).take_while(|b| b * b * b <= a3 - 16) {
            let b3 = b * b * b;

            for c in (b..).take_while(|c| c * c * c <= a3 - b3 - 8) {
                let c3 = c * c * c;
                let d = ((a3 - b3 - c3) as f64).cbrt() as i32;

                if d < c {
                    break;
                }

                if a3 == b3 + c3 + d * d * d {
                    println!("Cube = {a}, Triple = ({b},{c},{d})");
                    break;
                }
                // if a3 == b3 + c3 + (d + 1) * (d + 1) * (d + 1) {
                //     println!("Cube = {a}, Triple = ({b},{c},{})", d + 1);
                //     break;
                // }
            }
        }
    }
}
