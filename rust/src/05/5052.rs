// use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();
    let mut input = || input.next().unwrap();

    'outer: for _ in 0..parse_int(input()) {
        let n = parse_int(input());
        let mut numbers: Vec<_> = (0..n).map(|_| input()).collect();
        numbers.sort_unstable_by_key(|s| s.len());

        let mut number_set = HashSet::with_capacity(n);

        for number in numbers {
            for len in 1..number.len() {
                if number_set.contains(&number[..len]) {
                    println!("NO");
                    continue 'outer;
                }
            }

            number_set.insert(number);
        }

        println!("YES");

        // let mut trie = Trie::new();

        // for _ in 0..n {
        //     let number = input();

        //     if trie.is_prefix_exists {
        //         continue;
        //     }

        //     trie.insert(number);
        // }

        // println!("{}", if trie.is_prefix_exists { "NO" } else { "YES" });
    }
}

fn parse_int(buf: &str) -> usize {
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
