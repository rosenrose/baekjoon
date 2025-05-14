const OFFSET: u32 = '가' as u32;
const LAST_COUNT: u32 = '갛' as u32 - OFFSET + 1;
const MIDDLE_COUNT: u32 = ('깋' as u32 - OFFSET + 1) / LAST_COUNT;

fn main() {
    let consonants: Vec<_> = ('ㄱ'..='ㅎ').collect();
    let firsts: Vec<_> = consonants
        .iter()
        .filter(|ch| matches!(ch, 'ㄱ'..='ㄲ' | 'ㄴ' | 'ㄷ'..='ㄹ' | 'ㅁ'..='ㅃ' | 'ㅅ'..='ㅎ'))
        .collect();
    let lasts: Vec<_> = consonants
        .iter()
        .filter(|ch| !matches!(ch, 'ㄸ' | 'ㅃ' | 'ㅉ'))
        .collect();

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut code = buf.chars().nth(0).unwrap() as u32;
    code -= OFFSET;

    let first = firsts[(code / (LAST_COUNT * MIDDLE_COUNT)) as usize];
    code %= LAST_COUNT * MIDDLE_COUNT;

    let middle = char::from_u32('ㅏ' as u32 + (code / LAST_COUNT)).unwrap();
    code %= LAST_COUNT;

    println!("{first}\n{middle}");

    if code == 0 {
        println!("");
        return;
    }

    let last = lasts[(code - 1) as usize];

    println!("{last}");
}
/*
      초성 종성
ㄱ 0   0   1
ㄲ 1   1   2
ㄳ 2       3
ㄴ 3   2   4
ㄵ 4       5
ㄶ 5       6
ㄷ 6   3   7
ㄸ 7   4
ㄹ 8   5   8
ㄺ 9       9
ㄻ 10      10
ㄼ 11      11
ㄽ 12      12
ㄾ 13      13
ㄿ 14      14
ㅀ 15      15
ㅁ 16  6   16
ㅂ 17  7   17
ㅃ 18  8
ㅄ 19      18
ㅅ 20  9   19
ㅆ 21  10  20
ㅇ 22  11  21
ㅈ 23  12  22
ㅉ 24  13
ㅊ 25  14  23
ㅋ 26  15  24
ㅌ 27  16  25
ㅍ 28  17  26
ㅎ 29  18  27
*/
