fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    println!("{}", get_kmp_count(buf.trim(), "JOI"));
    println!("{}", get_kmp_count(buf.trim(), "IOI"));
}

fn get_kmp_count(source: &str, target: &str) -> i32 {
    let target: Vec<char> = target.chars().collect();
    let partial_match = get_partial_match(&target);

    let mut count = 0;
    let mut start = 0;

    for s in source.chars() {
        while target[start] != s && start > 0 {
            start = partial_match[start - 1];
        }

        if target[start] == s {
            if start < target.len() - 1 {
                start += 1;
            } else {
                count += 1;
                start = partial_match[start];
            }
        }
    }

    count
}

fn get_partial_match(chars: &Vec<char>) -> Vec<usize> {
    let mut partial_match = vec![0; chars.len()];
    let mut start = 0;

    for i in 1..chars.len() {
        while (chars[start] != chars[i]) && start > 0 {
            start = partial_match[start - 1];
        }

        if chars[start] == chars[i] {
            start += 1;
            partial_match[i] = start;
        }
    }

    partial_match
}
