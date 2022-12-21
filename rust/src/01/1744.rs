// use std::collections::HashMap;

// fn main() {
//     let mut buf = String::new();
//     std::io::stdin().read_line(&mut buf).unwrap();

//     let n: i32 = buf.trim().parse().unwrap();

//     let (mut path_1, mut path_2, mut path_3) = (vec![1], vec![1, 2], vec![1, 3]);
//     let mut cache = HashMap::from([
//         (1, (0, &mut path_1)),
//         (2, (1, &mut path_2)),
//         (3, (1, &mut path_3)),
//     ]);

//     let (count, path) = make_1_count(n, &mut cache);

//     println!("{count}");

//     for p in path.iter().rev() {
//         print!("{p} ");
//     }
// }

// fn make_1_count<'a>(
//     n: i32,
//     cache: &mut HashMap<i32, (i32, &'a mut Vec<i32>)>,
// ) -> (i32, &'a mut Vec<i32>) {
//     match cache.get(&n) {
//         Some(count_path) => count_path,
//         _ => {
//             let mut list = Vec::new();
//             let (count, mut path);

//             if n % 3 == 0 {
//                 (count, path) = make_1_count(n / 3, cache);
//                 list.push((count + 1, path));
//             }
//             if n % 2 == 0 {
//                 (count, path) = make_1_count(n / 2, cache);
//                 list.push((count + 1, path));
//             }
//             if (n - 1) % 3 == 0 || (n - 1) % 2 == 0 {
//                 (count, path) = make_1_count(n - 1, cache);
//                 list.push((count + 1, path));
//             }
//             if (n - 2) % 3 == 0 {
//                 (count, path) = make_1_count(n - 2, cache);
//                 path.push(n - 1);
//                 list.push((count + 2, path));
//             }
//             // println!("{n} {list:?}");
//             let (count, mut path) = list.iter().min().unwrap();

//             path.push(n);
//             cache.insert(n, (*count, path));

//             (*count, path)
//         }
//     }
// }
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf.lines().map(|s| s.parse::<i32>().unwrap());

    let mut nums: Vec<_> = input.skip(1).collect();
    nums.sort();

    let (mut sum, mut product_sum) = (0, 0);

    while let Some(&n) = nums.get(1) {
        if n > 0 {
            break;
        }

        product_sum += nums.remove(0) * nums.remove(0);
    }

    while let Some(&n) = nums.iter().nth_back(1) {
        if n <= 0 {
            break;
        }

        let (a, b) = (nums.pop().unwrap(), nums.pop().unwrap());

        if a == 1 || b == 1 {
            sum += a + b;
        } else {
            product_sum += a * b;
        }
    }

    sum += nums.iter().sum::<i32>();

    println!("{}", sum + product_sum);
}
