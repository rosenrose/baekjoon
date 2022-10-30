fn main() {
    let rows = [
        ["SHIP NAME", "CLASS", "DEPLOYMENT", "IN SERVICE"],
        ["N2 Bomber", "Heavy Fighter", "Limited", "21"],
        ["J-Type 327", "Light Combat", "Unlimited", "1"],
        ["NX Cruiser", "Medium Fighter", "Limited", "18"],
        ["N1 Starfighter ", "Medium Fighter", "Unlimited", "25"],
        ["Royal Cruiser", "Light Combat", "Limited", "4"],
    ];

    for row in rows {
        println!("{:15}{:15}{:11}{:10}", row[0], row[1], row[2], row[3]);
    }
}
