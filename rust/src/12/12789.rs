use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let mut stack1 = Vec::new();
    let mut stack2 = Vec::new();

    for num in input.skip(1) {
        if (stack1.is_empty() && num == 1)
            || matches!(stack1.last(), Some(&top1) if top1 == num - 1)
        {
            stack1.push(num);
        } else {
            stack2.push(num);
        }

        while let (Some(&top1), Some(&top2)) = (stack1.last(), stack2.last()) {
            if top1 + 1 != top2 {
                break;
            }

            stack1.push(stack2.pop().unwrap());
        }
    }

    if stack1
        .iter()
        .chain(stack2.iter().rev())
        .enumerate()
        .any(|(i, &num)| i as i32 + 1 != num)
    {
        println!("Sad");
        return;
    }

    println!("Nice");
}
