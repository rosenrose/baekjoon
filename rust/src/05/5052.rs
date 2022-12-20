// use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.lines();

    'outer: for _ in 0..parse_int(input.next().unwrap()) {
        let n = parse_int(input.next().unwrap());
        let mut numbers: Vec<_> = (0..n).map(|_| input.next().unwrap()).collect();
        numbers.sort_unstable_by_key(|s| s.len());

        let mut number_set = HashSet::new();

        for number in numbers {
            for len in 1..number.len() {
                if number_set.contains(&number[..len]) {
                    println!("NO");
                    continue 'outer;
                }
            }

            number_set.insert(number);
        }
        // println!("{prefixes:?}");
        println!("YES");

        // let mut trie = Trie::new();

        // for _ in 0..n {
        //     let number = input.next().unwrap();

        //     if trie.is_prefix_exists {
        //         continue;
        //     }

        //     trie.insert(number);
        // }

        // println!("{}", if trie.is_prefix_exists { "NO" } else { "YES" });
    }
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}

// #[derive(Default, Debug)]
// struct Node<'a> {
//     children: HashMap<char, Node<'a>>,
//     value: Option<&'a str>,
// }

// #[derive(Debug)]
// struct Trie<'a> {
//     root: Node<'a>,
//     is_prefix_exists: bool,
// }

// impl<'a> Trie<'a> {
//     fn new() -> Self {
//         Self {
//             root: Node::default(),
//             is_prefix_exists: false,
//         }
//     }

//     fn insert(&mut self, key: &'a str) {
//         let mut node = &mut self.root;

//         for c in key.chars() {
//             node = node.children.entry(c).or_insert(Node::default());

//             if node.value.is_some() {
//                 self.is_prefix_exists = true;
//             }
//         }

//         node.value = Some(key);

//         if !node.children.is_empty() {
//             self.is_prefix_exists = true;
//         }
//     }
// }