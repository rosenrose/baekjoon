use std::io::{stdin, Read};

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
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.lines().map(|s| s.chars().nth(0));

    let first = input.next().unwrap().unwrap();
    let medial = input.next().unwrap().unwrap();

    let mut code = '가' as u32;

    code += FIRSTS.iter().position(|&c| c == first).unwrap() as u32 * (MEDIAL_COUNT * LAST_COUNT);

    code += (medial as u32 - 'ㅏ' as u32) * LAST_COUNT;

    code += match input.next().unwrap() {
        Some(last) => LASTS.iter().position(|&c| c == last).unwrap() as u32 + 1,
        None => 0,
    };

    println!("{}", char::from_u32(code).unwrap());
}
