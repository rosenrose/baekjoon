use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    const ALBUMS: [(i32, &str); 25] = [
        (1967, "DavidBowie"),
        (1969, "SpaceOddity"),
        (1970, "TheManWhoSoldTheWorld"),
        (1971, "HunkyDory"),
        (1972, "TheRiseAndFallOfZiggyStardustAndTheSpidersFromMars"),
        (1973, "AladdinSane"),
        (1973, "PinUps"),
        (1974, "DiamondDogs"),
        (1975, "YoungAmericans"),
        (1976, "StationToStation"),
        (1977, "Low"),
        (1977, "Heroes"),
        (1979, "Lodger"),
        (1980, "ScaryMonstersAndSuperCreeps"),
        (1983, "LetsDance"),
        (1984, "Tonight"),
        (1987, "NeverLetMeDown"),
        (1993, "BlackTieWhiteNoise"),
        (1995, "1.Outside"),
        (1997, "Earthling"),
        (1999, "Hours"),
        (2002, "Heathen"),
        (2003, "Reality"),
        (2013, "TheNextDay"),
        (2016, "BlackStar"),
    ];

    for (s, e) in (0..input()).map(|_| (input(), input())) {
        let result: Vec<_> = ALBUMS
            .iter()
            .filter(|(year, _)| (s..=e).contains(year))
            .collect();

        writeln!(output, "{}", result.len()).unwrap();

        for (year, title) in result {
            writeln!(output, "{year} {title}").unwrap();
        }
    }

    print!("{output}")
}
