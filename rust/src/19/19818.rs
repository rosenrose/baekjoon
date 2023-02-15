use std::collections::HashMap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let cite_regex = Regex::new(r"\\cite\{[a-z]+\}", false);
    let biblio_regex = Regex::new(r"\\bibitem\{[a-z]+\}", false);
    const BIBLIO_BEGIN: &str = r"\begin{thebibliography}{99}";
    const BIBLIO_END: &str = r"\end{thebibliography}";

    let mut cites = Vec::new();
    let mut bibitems = HashMap::new();
    let mut is_correct = true;

    for cite in input.by_ref().take_while(|&input| input != BIBLIO_BEGIN) {
        let mut cursor = 0;

        while let Some((mut start, mut end)) = cite_regex.find(&cite[cursor..]) {
            start += cursor;
            end += cursor;

            let refer = &cite[start + r"\cite{".len()..end - 1];
            cites.push(refer);

            cursor = end;
        }
    }

    for (i, biblio) in input.enumerate() {
        let mut cursor = 0;

        while let Some((mut start, mut end)) = biblio_regex.find(&biblio[cursor..]) {
            start += cursor;
            end += cursor;

            let refer = &biblio[start + r"\bibitem{".len()..end - 1];

            if refer != cites[i] {
                is_correct = false;
            }

            bibitems.insert(refer, biblio);
            cursor = end;
        }
    }
    // println!("{cites:?}\n{bibitems:?}");
    if is_correct {
        println!("Correct");
        return;
    }

    println!("Incorrect");
    println!("{BIBLIO_BEGIN}");

    for refer in cites {
        println!("{}", bibitems[refer]);
    }

    println!("{BIBLIO_END}");
}
