fn main() {
    for a in 6..=100_i32 {
        let a3 = a.pow(3);

        for b in (2_i32..).take_while(|b| b.pow(3) <= a3 - 16) {
            let b3 = b.pow(3);

            for c in (b..).take_while(|c| c.pow(3) <= a3 - b3 - 8) {
                let c3 = c.pow(3);
                let d = ((a3 - b3 - c3) as f64).powf(1.0 / 3.0) as i32;

                if d + 1 < c {
                    break;
                }

                if a3 == b3 + c3 + d.pow(3) {
                    println!("Cube = {a}, Triple = ({b},{c},{d})");
                    break;
                }
                if a3 == b3 + c3 + (d + 1).pow(3) {
                    println!("Cube = {a}, Triple = ({b},{c},{})", d + 1);
                    break;
                }
            }
        }
    }
}
