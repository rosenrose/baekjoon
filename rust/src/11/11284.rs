fn main() {
    const FIRSTS: [char; 19] = [
        'ㄱ', 'ㄲ', 'ㄴ', 'ㄷ', 'ㄸ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅃ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅉ', 'ㅊ',
        'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ',
    ];
    const LASTS: [char; 27] = [
        'ㄱ', 'ㄲ', 'ㄳ', 'ㄴ', 'ㄵ', 'ㄶ', 'ㄷ', 'ㄹ', 'ㄺ', 'ㄻ', 'ㄼ', 'ㄽ', 'ㄾ', 'ㄿ', 'ㅀ',
        'ㅁ', 'ㅂ', 'ㅄ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ',
    ];
    const MEDIAL_COUNT: u32 = 21;
    const LAST_COUNT: u32 = LASTS.len() as u32 + 1;

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut code = buf.chars().nth(0).unwrap() as u32;
    code -= '가' as u32;

    let first = FIRSTS[(code / (MEDIAL_COUNT * LAST_COUNT)) as usize];
    let first = char::from_u32(first as u32).unwrap();
    code %= MEDIAL_COUNT * LAST_COUNT;

    let medial = char::from_u32('ㅏ' as u32 + (code / LAST_COUNT)).unwrap();
    code %= LAST_COUNT;

    println!("{first}\n{medial}");

    if code == 0 {
        println!("");
    } else {
        let last = LASTS[(code - 1) as usize];
        let last = char::from_u32(last as u32).unwrap();

        println!("{last}");
    }
}
